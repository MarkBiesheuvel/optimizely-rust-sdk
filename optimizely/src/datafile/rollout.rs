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
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn experiments(&self) -> &Vec<Experiment> {
        &self.experiments
    }
}

#[derive(Debug)]
pub struct RolloutMap(HashMap<String, Rollout>);

impl<'de> Deserialize<'de> for RolloutMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map = HashMap::new();
        for rollout in Vec::<Rollout>::deserialize(deserializer)? {
            map.insert(rollout.id.clone(), rollout);
        }

        Ok(Self(map))
    }
}

impl RolloutMap {
    pub fn get(&self, id: &str) -> Option<&Rollout> {
        self.0.get(id)
    }
}
