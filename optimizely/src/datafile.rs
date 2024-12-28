//! Parsing the Optimizely datafile

// External imports
use error_stack::{Result, ResultExt};

// Relative imports of sub modules
use attribute::Attribute;
use environment::Environment;
pub use error::DatafileError;
pub(crate) use event::Event;
pub(crate) use experiment::Experiment;
pub(crate) use feature_flag::FeatureFlag;
use rollout::Rollout;
use traffic_allocation::TrafficAllocation;
pub(crate) use variation::Variation;

mod attribute;
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
