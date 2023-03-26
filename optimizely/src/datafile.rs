//! Parsing the Optimizely datafile

// External imports
use error_stack::{IntoReport, Result, ResultExt};
use std::collections::HashMap;

// Relative imports of sub modules
pub use error::DatafileError;
#[cfg(feature = "online")]
pub(crate) use event::Event;
pub(crate) use experiment::Experiment;
pub(crate) use feature_flag::FeatureFlag;
pub use json::Json;
pub(crate) use rollout::Rollout;
pub(crate) use traffic_allocation::TrafficAllocation;
pub(crate) use variation::Variation;

mod error;
#[cfg(feature = "online")]
mod event;
mod experiment;
mod feature_flag;
mod json;
mod rollout;
mod traffic_allocation;
mod variation;

#[derive(Debug)]
/// Contains all the settings, suchs as feature flags, events, and audiences
pub struct Datafile {
    account_id: String,
    revision: u32,
    feature_flags: HashMap<String, FeatureFlag>,
    #[cfg(feature = "online")]
    events: HashMap<String, Event>,
}

impl Datafile {
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

        Ok(Datafile {
            account_id,
            revision,
            feature_flags,
            #[cfg(feature = "online")]
            events,
        })
    }

    /// Account ID of the datafile
    pub fn account_id(&self) -> &str {
        &self.account_id
    }

    /// Revision of the datafile
    pub fn revision(&self) -> u32 {
        self.revision
    }

    /// Find a specific flag
    pub fn get_flag(&self, flag_key: &str) -> Option<&FeatureFlag> {
        self.feature_flags.get(flag_key)
    }

    /// Find a specific event
    #[cfg(feature = "online")]
    pub fn get_event(&self, event_key: &str) -> Option<&Event> {
        self.events.get(event_key)
    }
}
