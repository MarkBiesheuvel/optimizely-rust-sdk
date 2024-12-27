//! Parsing the Optimizely datafile

// External imports
use error_stack::{Result, ResultExt};

// Relative imports of sub modules
use environment::Environment;
pub use error::DatafileError;
pub(crate) use event::Event;
pub(crate) use experiment::Experiment;
pub(crate) use feature_flag::FeatureFlag;
use rollout::Rollout;
use traffic_allocation::TrafficAllocation;
pub(crate) use variation::Variation;

mod environment;
mod error;
mod event;
mod experiment;
mod feature_flag;
mod rollout;
mod traffic_allocation;
mod variation;

/// The datafile contains all the feature flags, experiments, events and other configuration from an Optimizely account.
///
/// This configuration is stored in JSON format.
/// A string containing this JSON format is used to build a `Datafile` struct.
/// The `serde_json` library is used to parse the JSON string into an hierarchy of Rust structs.
///
/// While it is possible to perform zero-copy deserialization with `serde`, it would require to store an owned `String`
/// containing the `content`.
/// This would mean that a lot of memory would stay allocated for JSON syntax and unused properties.
/// Instead the relevant fields are copied into their own `String`s.
#[derive(Debug)]
pub struct Datafile(Environment);

impl Datafile {
    /// Construct a new Datafile from a string containing a JSON document
    pub fn build(content: &str) -> Result<Datafile, DatafileError> {
        // Parse the JSON content via Serde into Rust structs
        let environment: Environment = serde_json::from_str(content).change_context(DatafileError::InvalidJson)?;
    /// Creates an empty Datafile struct
    pub fn new<T: Into<String>>(account_id: T, revision: u32) -> Datafile {
        Datafile {
            account_id: account_id.into(),
            revision,
            feature_flags: HashMap::new(),
            #[cfg(feature = "online")]
            events: HashMap::new(),
        }
    }

    /// Create Datafile from JSON
    pub fn build(json: &mut Json) -> Result<Datafile, DatafileError> {
        // Get account_id as String
        let account_id = json.get("accountId")?.as_string()?;

        // Get revision as String, ...
        let revision = json.get("revision")?.as_string()?;

        // ... and parse as u32
        let revision = revision
            .parse()
            .into_report()
            .change_context(DatafileError::InvalidRevision(revision))?;

        #[cfg(feature = "online")]
        let events = json
            .get("events")?
            .as_array()?
            .map(|mut json| Event::build(&mut json))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|event| (event.key().to_owned(), event))
            .collect::<HashMap<_, _>>();

        // Get HashMap of Rollouts
        let mut rollouts = json
            .get("rollouts")?
            .as_array()?
            .map(|mut json| Rollout::build(&mut json))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|rollout| (rollout.id().to_owned(), rollout))
            .collect::<HashMap<_, _>>();

        // Get HashMap of Experiments
        let mut experiments = json
            .get("experiments")?
            .as_array()?
            .map(|mut json| Experiment::build(&mut json))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|experiment| (experiment.id().to_owned(), experiment))
            .collect::<HashMap<_, _>>();

        // // Get Vec of feature flags
        let feature_flags = json
            .get("featureFlags")?
            .as_array()?
            .map(|mut json| FeatureFlag::build(&mut json, &mut rollouts, &mut experiments))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|flag| (flag.key().to_owned(), flag))
            .collect::<HashMap<_, _>>();

        Ok(Datafile(environment))
    }

    /// Get the account ID
    pub fn account_id(&self) -> &str {
        self.0.account_id()
    }

    /// Get the revision of the datafile
    pub fn revision(&self) -> u32 {
        self.0.revision()
    }

    /// Get the flag with the given key
    pub fn flag(&self, flag_key: &str) -> Option<&FeatureFlag> {
        self.0.feature_flags().get(flag_key)
    }

    /// Get the experiment with the given experiment ID
    pub fn experiment(&self, experiment_id: &str) -> Option<&Experiment> {
        self.0.experiments().get(experiment_id)
    }

    /// Get the rollout with the given rollout ID
    pub fn rollout(&self, rollout_id: &str) -> Option<&Rollout> {
        self.0.rollouts().get(rollout_id)
    }

    /// Get the event with the given key
    pub fn event(&self, event_key: &str) -> Option<&Event> {
        self.0.events().get(event_key)
    }
}
