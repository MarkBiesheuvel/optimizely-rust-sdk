// External imports
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

/// Optimizely feature flag.
#[derive(Deserialize, Debug)]
pub struct FeatureFlag {
    #[serde()]
    key: String,
    #[serde(rename = "rolloutId")]
    rollout_id: String,
    #[serde(rename = "experimentIds")]
    experiment_ids: Vec<String>,
    // TODO: variables
}

impl FeatureFlag {
    // Method to deserialize an array of Rollouts into a Hashmap of Rollouts
    pub fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<String, FeatureFlag>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map = HashMap::new();
        for flag in Vec::<FeatureFlag>::deserialize(deserializer)? {
            map.insert(flag.key.clone(), flag);
        }
        Ok(map)
    }

    #[allow(dead_code)]
    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn rollout_id(&self) -> &str {
        &self.rollout_id
    }

    pub fn experiments_ids(&self) -> &Vec<String> {
        &self.experiment_ids
    }
}
