//! Parsing the Optimizely datafile

use error_stack::{Report, ResultExt};
use std::ops::Deref;

// Relative imports of sub modules
pub(crate) use attribute::{Attribute, AttributeMap};
use audience::AudienceMap;
use environment::Environment;
pub use error::DatafileError;
pub(crate) use event::{Event, EventMap};
pub(crate) use experiment::{Experiment, ExperimentMap};
pub(crate) use feature_flag::{FeatureFlag, FeatureFlagMap};
use revision::Revision;
use rollout::RolloutMap;
use traffic_allocation::TrafficAllocation;
pub(crate) use variation::{Variation, VariationMap};

mod attribute;
mod audience;
mod environment;
mod error;
mod event;
mod experiment;
mod feature_flag;
mod revision;
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

impl TryFrom<&str> for Datafile {
    type Error = Report<DatafileError>;

    /// Construct a new Datafile from a string containing a JSON document
    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        // Parse the JSON content via Serde into Rust structs
        let environment = serde_json::from_str(value).change_context(DatafileError::InvalidJson)?;

        Ok(Datafile(environment))
    }
}

impl Deref for Datafile {
    type Target = Environment;

    /// Since a Datafile always contains exactly one environment, they can be used interchangeably.
    /// Therefore it makes sense to dereference a datafile to an environment in order to access its methods.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
