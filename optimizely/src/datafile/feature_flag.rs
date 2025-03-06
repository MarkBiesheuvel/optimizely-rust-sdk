// External imports
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

/// Optimizely feature flag.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FeatureFlag {
    key: String,
    rollout_id: String,
    experiment_ids: Vec<String>,
    // TODO: variables
}

impl FeatureFlag {
    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn rollout_id(&self) -> &str {
        &self.rollout_id
    }

    pub fn experiments_ids(&self) -> &[String] {
        &self.experiment_ids
    }
}

#[derive(Debug)]
pub struct FeatureFlagMap(HashMap<String, FeatureFlag>);

impl<'de> Deserialize<'de> for FeatureFlagMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map = HashMap::new();
        for flag in Vec::<FeatureFlag>::deserialize(deserializer)? {
            map.insert(flag.key.clone(), flag);
        }

        Ok(Self(map))
    }
}

impl FeatureFlagMap {
    pub fn get(&self, key: &str) -> Option<&FeatureFlag> {
        self.0.get(key)
    }
}
