//! Result of a feature flag

// Relative imports of sub modules
pub use decide_options::DecideOptions;

use crate::datafile;
mod decide_options;

/// Decision for a specific user and feature flag
#[derive(Debug, Clone)]
pub struct Decision<'a, 'b> {
    flag_key: &'a str,
    campaign_id: &'b str,
    experiment_id: &'b str,
    variation_id: &'b str,
    variation_key: &'b str,
    enabled: bool,
}

impl Decision<'_, '_> {
    pub(crate) fn from<'a, 'b>(
        flag_key: &'a str, experiment: &'b datafile::Experiment, variation: &'b datafile::Variation,
    ) -> Decision<'a, 'b> {
        Decision {
            flag_key,
            campaign_id: experiment.campaign_id(),
            experiment_id: experiment.id(),
            variation_id: variation.id(),
            variation_key: variation.key(),
            enabled: variation.is_feature_enabled(),
        }
    }

    pub(crate) fn off<'a>(flag_key: &'a str) -> Decision<'a, 'static> {
        Decision {
            flag_key: flag_key,
            campaign_id: "",
            experiment_id: "",
            variation_id: "",
            variation_key: "off",
            enabled: false,
        }
    }
}

impl<'a, 'b> Decision<'a, 'b> {
    /// Get the flag key for which this decision was made
    pub fn flag_key(&self) -> &'a str {
        &self.flag_key
    }

    /// Get whether the flag should be enabled or disable
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    /// Get the campaign ID
    pub fn campaign_id(&self) -> &'b str {
        &self.campaign_id
    }

    /// Get the experiment ID
    pub fn experiment_id(&self) -> &'b str {
        &self.experiment_id
    }

    /// Get the variation ID that was decided
    pub fn variation_id(&self) -> &'b str {
        &self.variation_id
    }

    /// Get the variation key that was decided
    pub fn variation_key(&self) -> &'b str {
        &self.variation_key
    }
}
