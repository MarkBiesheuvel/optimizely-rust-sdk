//! Result of a feature flag

// Relative imports of sub modules
pub use decide_options::DecideOptions;

use crate::datafile;
mod decide_options;

/// Decision for a specific user and feature flag
///
/// Unfortunately, references to the datafile have to be cloned in order to release the read/write lock
#[derive(Debug, Clone)]
pub struct Decision<'a> {
    flag_key: &'a str,
    campaign_id: String,
    experiment_id: String,
    variation_id: String,
    variation_key: String,
    enabled: bool,
}

impl Decision<'_> {
    pub(crate) fn from<'a>(
        flag_key: &'a str, experiment: &datafile::Experiment, variation: &datafile::Variation,
    ) -> Decision<'a> {
        Decision {
            flag_key,
            campaign_id: experiment.campaign_id().into(),
            experiment_id: experiment.id().into(),
            variation_id: variation.id().into(),
            variation_key: variation.key().into(),
            enabled: variation.is_feature_enabled(),
        }
    }

    pub(crate) fn off<'a>(flag_key: &'a str) -> Decision<'a> {
        Decision {
            flag_key: flag_key,
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
