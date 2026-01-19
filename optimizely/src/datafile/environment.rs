// External imports
use serde::Deserialize;

// Imports from super
use super::{
    audience::Audience, rollout::Rollout, Attribute, AttributeMap, AudienceMap, Event, EventMap, Experiment,
    ExperimentMap, FeatureFlag, FeatureFlagMap, Revision, RolloutMap,
};

/// Each Datafile is for exactly one Environment, so most methods are implemented on Environment instead of Datafile
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Environment {
    account_id: String,
    project_id: String,
    environment_key: String,
    sdk_key: String,
    revision: Revision,
    bot_filtering: bool,
    #[serde(rename = "anonymizeIP")]
    anonymize_ip: bool,
    events: EventMap,
    attributes: AttributeMap,
    #[serde(rename = "typedAudiences")]
    #[allow(dead_code)]
    audiences: AudienceMap,
    experiments: ExperimentMap,
    rollouts: RolloutMap,
    feature_flags: FeatureFlagMap,
}

impl Environment {
    /// Getter for `account_id` field
    pub fn account_id(&self) -> &str {
        &self.account_id
    }

    /// Getter for `sdk_key` field
    pub fn sdk_key(&self) -> &str {
        &self.sdk_key
    }

    #[allow(dead_code)]
    pub fn project_id(&self) -> &str {
        &self.project_id
    }

    #[allow(dead_code)]
    pub fn environment_key(&self) -> &str {
        &self.environment_key
    }

    /// Getter for `revision` field
    pub fn revision(&self) -> u32 {
        *self.revision
    }

    #[allow(dead_code)]
    pub fn bot_filtering(&self) -> bool {
        self.bot_filtering
    }

    #[allow(dead_code)]
    pub fn anonymize_ip(&self) -> bool {
        self.anonymize_ip
    }

    /// Get the flag with the given key
    pub(crate) fn flag(&self, flag_key: &str) -> Option<&FeatureFlag> {
        self.feature_flags.get(flag_key).or_else(|| {
            log::warn!("Flag key '{flag_key}' does not exist in datafile");
            None
        })
    }

    /// Get the experiment with the given experiment ID
    pub(crate) fn experiment(&self, experiment_id: &str) -> Option<&Experiment> {
        self.experiments.get(experiment_id).or_else(|| {
            log::warn!("Experiment ID '{experiment_id}' does not exist in datafile");
            None
        })
    }

    /// Get the rollout with the given rollout ID
    pub(crate) fn rollout(&self, rollout_id: &str) -> Option<&Rollout> {
        self.rollouts.get(rollout_id).or_else(|| {
            log::warn!("Rollout ID '{rollout_id}' does not exist in datafile");
            None
        })
    }

    /// Get the event with the given key
    pub(crate) fn event(&self, event_key: &str) -> Option<&Event> {
        self.events.get(event_key).or_else(|| {
            log::warn!("Event key '{event_key}' does not exist in datafile");
            None
        })
    }

    /// Get the attribute with the given key
    pub(crate) fn attribute(&self, attribute_key: &str) -> Option<&Attribute> {
        self.attributes.get(attribute_key).or_else(|| {
            log::warn!("Attribute key '{attribute_key}' does not exist in datafile");
            None
        })
    }

    /// Get the audience with the given audience ID
    pub(crate) fn audience(&self, audience_id: &str) -> Option<&Audience> {
        self.audiences.get(audience_id).or_else(|| {
            log::warn!("Audience id '{audience_id}' does not exist in datafile");
            None
        })
    }
}
