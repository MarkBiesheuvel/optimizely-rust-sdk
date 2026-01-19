//! Result of a feature flag

// External imports
use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::datafile::{Experiment, FeatureFlag, Variation};

/// Decision for a specific user and feature flag
///
/// Unfortunately, references to the datafile have to be cloned in order to release the read/write lock
#[derive(Debug, Clone)]
pub struct Decision {
    flag_key: String,
    campaign_id: String,
    experiment_id: String,
    variation_id: String,
    variation_key: String,
    enabled: bool,
}

impl Decision {
    pub(crate) fn from(flag: &FeatureFlag, experiment: &Experiment, variation: &Variation) -> Decision {
        // Unfortunately, we will have to clone all Strings in order to release the read/write lock on the Datafile
        Decision {
            flag_key: flag.key().into(),
            campaign_id: experiment.campaign_id().into(),
            experiment_id: experiment.id().into(),
            variation_id: variation.id().into(),
            variation_key: variation.key().into(),
            enabled: variation.is_feature_enabled(),
        }
    }

    pub(crate) fn off(flag_key: &str) -> Decision {
        Decision {
            flag_key: flag_key.into(),
            campaign_id: String::default(),
            experiment_id: String::default(),
            variation_id: String::default(),
            variation_key: String::from("off"),
            enabled: false,
        }
    }

    /// Get the flag key for which this decision was made
    pub fn flag_key(&self) -> &str {
        &self.flag_key
    }

    /// Get whether the flag should be enabled or disable
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    /// Get the campaign ID
    pub fn campaign_id(&self) -> &str {
        &self.campaign_id
    }

    /// Get the experiment ID
    pub fn experiment_id(&self) -> &str {
        &self.experiment_id
    }

    /// Get the variation ID that was decided
    pub fn variation_id(&self) -> &str {
        &self.variation_id
    }

    /// Get the variation key that was decided
    pub fn variation_key(&self) -> &str {
        &self.variation_key
    }
}

impl Serialize for Decision {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("Decision", 4)?;
        st.serialize_field("campaign_id", self.campaign_id())?;
        st.serialize_field("experiment_id", self.experiment_id())?;
        st.serialize_field("variation_id", self.variation_id())?;
        st.serialize_field("is_campaign_holdback", &false)?;
        st.end()
    }
}
