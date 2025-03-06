// External imports
use serde::Deserialize;

// Imports from super
use super::{AttributeMap, AudienceMap, EventMap, ExperimentMap, FeatureFlagMap, Revision, RolloutMap};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Environment {
    account_id: String,
    project_id: String,
    environment_key: String,
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

    #[allow(dead_code)]
    pub fn project_id(&self) -> &str {
        &self.project_id
    }

    #[allow(dead_code)]
    pub fn environment_key(&self) -> &str {
        &self.environment_key
    }

    /// Getter for `revision` field
    pub fn revision(&self) -> &Revision {
        &self.revision
    }

    #[allow(dead_code)]
    pub fn bot_filtering(&self) -> bool {
        self.bot_filtering
    }

    #[allow(dead_code)]
    pub fn anonymize_ip(&self) -> bool {
        self.anonymize_ip
    }

    pub fn feature_flags(&self) -> &FeatureFlagMap {
        &self.feature_flags
    }

    pub fn experiments(&self) -> &ExperimentMap {
        &self.experiments
    }

    pub fn rollouts(&self) -> &RolloutMap {
        &self.rollouts
    }

    pub fn events(&self) -> &EventMap {
        &self.events
    }

    pub fn attributes(&self) -> &AttributeMap {
        &self.attributes
    }

    pub fn audiences(&self) -> &AudienceMap {
        &self.audiences
    }
}
