//! Parsing the Optimizely datafile
use std::fs::File;
use std::io::Read;

use crate::error::DatafileError;
use error_stack::{Report, Result, ResultExt};
use std::ops::Deref;

// Relative imports of sub modules
pub(crate) use attribute::{Attribute, AttributeMap};
use audience::AudienceMap;
use environment::Environment;
#[cfg(feature = "online")]
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
#[cfg(feature = "online")]
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
/// The `serde_json` crate is used to parse the JSON string into an hierarchy of Rust structs.
///
/// While it is possible to perform zero-copy deserialization with `serde`, there are two main issues.
/// First, the JSON string is read from disk or read from an HTTP response by the client, so the client owns the String.
/// The client cannot store an owned String as well as borrowed references to that String.
/// This would require unsafe code using raw pointers or using Pin.
/// Secondly, keeping the entire JSON string in memory would use up a lot of additional space for JSON syntax and
/// unused properties. Instead, only the relevant fields are copied into owned Strings.
#[derive(Debug)]
pub struct Datafile(Environment);

impl Datafile {
    /// Download the datafile from the CDN using an SDK key
    #[cfg(feature = "online")]
    pub fn from_sdk_key(sdk_key: &str) -> Result<Datafile, DatafileError> {
        // Construct URL
        let url = format!("https://cdn.optimizely.com/datafiles/{}.json", sdk_key);

        // Make GET request
        let response = ureq::get(&url)
            .call()
            .change_context(DatafileError::FailedRequest)?;

        // Get response body
        let content = response
            .into_string()
            .change_context(DatafileError::FailedResponse)?;

        // Use response to build Client
        Datafile::from_string(content)
    }

    /// Read the datafile from the local filesystem
    pub fn from_local_datafile(file_path: &str) -> Result<Datafile, DatafileError> {
        // Read content from local path
        let mut content = String::new();

        // Open file
        let mut file = File::open(file_path).change_context(DatafileError::FailedFileOpen)?;

        // Read file content into String
        file.read_to_string(&mut content)
            .change_context(DatafileError::FailedFileRead)?;

        // Use file content to build Client
        Datafile::from_string(content)
    }

    /// Use a string variable as the datafile
    pub fn from_string<S>(content: S) -> Result<Datafile, DatafileError>
    where
        S: AsRef<str>,
    {
        Datafile::try_from(content.as_ref())
    }
}

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
