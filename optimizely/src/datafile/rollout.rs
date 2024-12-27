// External imports
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

// Imports from super
use super::Experiment;

#[derive(Deserialize, Debug)]
pub struct Rollout {
    id: String,
    experiments: Vec<Experiment>,
}

impl Rollout {
    // Method to deserialize an array of Rollouts into a Hashmap of Rollouts
    pub fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<String, Rollout>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map = HashMap::new();
        for rollout in Vec::<Rollout>::deserialize(deserializer)? {
            map.insert(rollout.id.clone(), rollout);
        }
        Ok(map)
    }

    #[allow(dead_code)]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[allow(dead_code)]
    pub fn experiments(&self) -> &Vec<Experiment> {
        &self.experiments
    }
}
