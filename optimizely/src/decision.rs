//! Result of a feature flag

// Relative imports of sub modules
pub use decide_options::DecideOptions;
mod decide_options;

/// Decision for a specific user and feature flag
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
    pub(crate) fn new<T: Into<String>>(
        flag_key: T, campaign_id: T, experiment_id: T, variation_id: T, variation_key: T, enabled: bool,
    ) -> Decision {
        Decision {
            flag_key: flag_key.into(),
            campaign_id: campaign_id.into(),
            experiment_id: experiment_id.into(),
            variation_id: variation_id.into(),
            variation_key: variation_key.into(),
            enabled,
        }
    }

    pub(crate) fn off(flag_key: &str) -> Decision {
        Decision::new(flag_key, "-1", "-1", "-1", "off", false)
    }

    /// Get the flag key for which this decision was made
    pub fn flag_key(&self) -> &str {
        &self.flag_key
    }

    /// Get whether the flag should be enabled or disable
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    /// Get the variation key that was decided
    pub fn variation_key(&self) -> &str {
        &self.variation_key
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
}
