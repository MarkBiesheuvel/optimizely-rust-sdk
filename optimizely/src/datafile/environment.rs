// External imports
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

// Imports from super
use super::{Event, Experiment, FeatureFlag, Rollout};

#[derive(Deserialize, Debug)]
pub struct Environment {
    #[serde(rename = "accountId")]
    account_id: String,
    #[serde(rename = "projectId")]
    project_id: String,
    #[serde(rename = "environmentKey")]
    environment_key: String,
    #[serde(deserialize_with = "deserialize_revision")]
    revision: u32,
    #[serde(rename = "botFiltering")]
    bot_filtering: bool,
    #[serde(rename = "anonymizeIP")]
    anonymize_ip: bool,
    #[serde(rename = "events", deserialize_with = "Event::deserialize")]
    events: HashMap<String, Event>,
    #[serde(deserialize_with = "Experiment::deserialize")]
    experiments: HashMap<String, Experiment>,
    #[serde(deserialize_with = "Rollout::deserialize")]
    rollouts: HashMap<String, Rollout>,
    #[serde(rename = "featureFlags", deserialize_with = "FeatureFlag::deserialize")]
    feature_flags: HashMap<String, FeatureFlag>,
}

fn deserialize_revision<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(deserializer)?
        .parse::<u32>()
        .map_err(serde::de::Error::custom)
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
    pub fn revision(&self) -> u32 {
        self.revision
    }

    #[allow(dead_code)]
    pub fn bot_filtering(&self) -> bool {
        self.bot_filtering
    }

    #[allow(dead_code)]
    pub fn anonymize_ip(&self) -> bool {
        self.anonymize_ip
    }

    pub fn feature_flags(&self) -> &HashMap<String, FeatureFlag> {
        &self.feature_flags
    }

    pub fn experiments(&self) -> &HashMap<String, Experiment> {
        &self.experiments
    }

    pub fn rollouts(&self) -> &HashMap<String, Rollout> {
        &self.rollouts
    }

    pub fn events(&self) -> &HashMap<String, Event> {
        &self.events
    }
}
