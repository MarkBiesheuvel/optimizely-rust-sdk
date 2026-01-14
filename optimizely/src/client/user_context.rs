// External imports
use murmur3::murmur3_32 as murmur3_hash;
use std::collections::HashMap;

// Imports from crate
use crate::datafile::{Experiment, FeatureFlag};
use crate::{AttributeValue, Conversion, DecideOptions, Decision};

// Imports from super
use super::{
    user_attribute::{UserAttribute, UserAttributeMap},
    Client, DatafileReadLock,
};

/// Constant used for the hashing algorithm
const HASH_SEED: u32 = 1;

/// Hash values are between 0 and u32::MAX (inclusive) or 2^32 (exclusive)
const MAX_HASH_VALUE: f64 = 4_294_967_296_f64;

/// Range values are between 0 and 10_000 (exclusive)
const MAX_RANGE_VALUE: f64 = 10_000_f64;

/// A user-specific context of the SDK client
///
/// ```
/// use optimizely::{Client, DecideOptions};
///
/// // Initialize Optimizely client using local datafile
/// let file_path = "../datafiles/sandbox.json";
/// let optimizely_client = Client::from_local_datafile(file_path)?
///     .initialize();
///
/// // Do not send any decision events
/// let decide_options = DecideOptions {
///     disable_decision_event: true,
///     ..DecideOptions::default()
/// };
///
/// // Create a user context
/// let user_context = optimizely_client.create_user_context("123abc789xyz");
///
/// // Decide a feature flag for this user
/// let decision = user_context.decide_with_options("buy_button", &decide_options);
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct UserContext<'a> {
    client: &'a Client,
    user_id: &'a str,
    user_attributes: UserAttributeMap,
}

impl<'a> UserContext<'a> {
    // Only allow UserContext to be constructed from a Client
    pub(crate) fn new(client: &'a Client, user_id: &'a str) -> UserContext<'a> {
        UserContext {
            client,
            user_id,
            user_attributes: UserAttributeMap::default(),
        }
    }

    /// Add a new attribute to a user context
    /// Note that the type is not verified,
    ///     since the same attribute might be used in different conditions using different type comparisons
    ///
    /// TODO: implement type specific versions of this function, such as set_boolean_attribute
    pub fn set_attribute(&mut self, key: impl Into<String>, value: AttributeValue) {
        let key = key.into();
        if let Some(datafile_attribute) = self.client.datafile().attribute(&key) {
            // Create user attribute by combining a value to a datafile attribute
            let user_attribute = UserAttribute::from_attribute_and_value(datafile_attribute, value);
            self.user_attributes.insert(key, user_attribute);
        }
    }
}

impl UserContext<'_> {
    /// Get the client instance
    pub fn client(&self) -> &Client {
        self.client
    }

    /// Get the id of a user
    pub fn user_id(&self) -> &str {
        self.user_id
    }

    /// Get all attributes of a user
    pub fn user_attributes(&self) -> Vec<&UserAttribute> {
        self.user_attributes.values().collect()
    }

    #[cfg(feature = "online")]
    /// Track a conversion event (without properties and tags) for this user
    pub fn track_event(&self, event_key: &str) {
        let properties = HashMap::default();
        let tags = HashMap::default();
        self.track_event_with_properties_and_tags(event_key, properties, tags)
    }

    #[cfg(feature = "online")]
    /// Track a conversion event with properties (but without tags) for this user
    pub fn track_event_with_properties(&self, event_key: &str, properties: HashMap<String, String>) {
        let tags = HashMap::default();
        self.track_event_with_properties_and_tags(event_key, properties, tags)
    }

    #[cfg(feature = "online")]
    /// Track a conversion event with properties and tags for this user
    pub fn track_event_with_properties_and_tags(
        &self, event_key: &str, properties: HashMap<String, String>, tags: HashMap<String, String>,
    ) {
        // Find the event key in the datafile
        if let Some(event) = self.client.datafile().event(event_key) {
            log::debug!("Logging conversion event");

            // Create conversion to send to dispatcher
            let conversion = Conversion::new(event_key, event.id(), properties, tags);

            // Ignore result of the send_decision function
            self.client
                .event_dispatcher()
                .send_conversion_event(self, &conversion);
        }
    }

    /// Decide which variation to show to a user
    pub fn decide<'a>(&self, flag_key: &'a str) -> Decision<'a> {
        let options = self.client().default_decide_options();
        println!("{:?}", flag_key);
        self.decide_with_options(flag_key, &options)
    }

    /// Decide which variation to show to a user
    pub fn decide_with_options<'a>(&self, flag_key: &'a str, options: &DecideOptions) -> Decision<'a> {
        // Acquire datafile read lock
        let datafile = self.client.datafile();

        // Retrieve Flag
        let flag = match datafile.flag(flag_key) {
            Some(flag) => flag,
            None => {
                // When flag key cannot be found, return the off variation
                // CONSIDERATION: Could have used Result<Decision, E> but this is how other Optimizely SDKs work
                return Decision::off(flag_key);
            }
        };

        // Only send decision events if the disable_decision_event option is false
        let mut send_decision = !options.disable_decision_event;

        // Get the selected variation for the given flag
        let decision = self
            .decide_for_flag(&datafile, flag_key, flag, &mut send_decision)
            .unwrap_or_else(|| Decision::off(flag_key));

        #[cfg(feature = "online")]
        if send_decision {
            self.client
                .event_dispatcher()
                .send_decision_event(self, &decision);
        }

        // Return
        decision
    }

    fn decide_for_flag<'a, 'b>(
        &'b self, datafile: &'b DatafileReadLock<'b>, flag_key: &'a str, flag: &'b FeatureFlag,
        send_decision: &mut bool,
    ) -> Option<Decision<'a>> {
        // Find first Experiment for which this user qualifies, and then use that decision
        let decision = flag
            .experiments_ids()
            .iter()
            .filter_map(|experiment_id| datafile.experiment(experiment_id))
            .find(|experiment| self.is_in_target_audience(datafile, experiment))
            .and_then(|experiment| self.decide_for_experiment(flag_key, experiment));

        match decision {
            Some(_) => {
                // Send out a decision event for an A/B Test
                *send_decision &= true;

                decision
            }
            None => {
                // Do not send any decision for a Rollout (Targeted Delivery)
                *send_decision = false;

                // No direct experiment found, let's look at the Rollout
                let rollout = datafile.rollout(flag.rollout_id())?;

                // Find first Experiment for which this user qualifies, and then use that decision
                rollout
                    .experiments()
                    .iter()
                    .find(|experiment| self.is_in_target_audience(datafile, experiment))
                    .and_then(|experiment| self.decide_for_experiment(flag_key, experiment))
            }
        }
    }

    fn decide_for_experiment<'a, 'b>(&'b self, flag_key: &'a str, experiment: &'b Experiment) -> Option<Decision<'a>> {
        // Use references for the ids
        let user_id = self.user_id();
        let experiment_id = experiment.id();

        // Concatenate user id and experiment id
        let bucketing_key = format!("{user_id}{experiment_id}");

        // To hash the bucket key it needs to be converted to an array of `u8` bytes
        // Use Murmur3 (32-bit) with seed
        let mut bytes = bucketing_key.as_bytes();
        let hash_value = murmur3_hash(&mut bytes, HASH_SEED).unwrap();

        // Bring the hash into a range of 0 to 10_000
        let bucket_value = ((hash_value as f64) / MAX_HASH_VALUE * MAX_RANGE_VALUE) as u64;

        // Get the variation ID according to the traffic allocation
        experiment
            .traffic_allocation()
            .variation(bucket_value)
            // Map it to a Variation struct
            .and_then(|variation_id| experiment.variation(variation_id))
            // Combine it with the experiment
            .map(|variation| Decision::from(flag_key, experiment, variation))
    }

    fn is_in_target_audience<'b>(&'b self, datafile: &'b DatafileReadLock<'b>, experiment: &'b Experiment) -> bool {
        let audience_ids = experiment.audience_ids();

        // If there are no audiences, everyone is welcome
        if audience_ids.is_empty() {
            return true;
        }

        // Otherwise, the user needs to match at least one audience
        audience_ids.iter().any(|audience_id| {
            // Retrieve the audience from the datafile
            let audience = match datafile.audience(audience_id.as_ref()) {
                Some(audience) => audience,
                None => {
                    // Not found in datafile, so user does not match
                    return false;
                }
            };

            audience.condition().does_match(&self.user_attributes)
        })
    }
}
