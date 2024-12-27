// External imports
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

// Imports from super
use super::{TrafficAllocation, Variation};

#[derive(Deserialize, Debug)]
pub struct Experiment {
    #[serde()]
    id: String,
    #[serde(rename = "layerId")]
    campaign_id: String,
    #[serde(rename = "trafficAllocation", deserialize_with = "TrafficAllocation::deserialize")]
    traffic_allocation: TrafficAllocation,
    #[serde(rename = "variations", deserialize_with = "Variation::deserialize")]
    variations: HashMap<String, Variation>,
}

impl Experiment {
    // Method to deserialize an array of Experiments into a Hashmap of Experiments
    pub fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<String, Experiment>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map = HashMap::new();
        for experiment in Vec::<Experiment>::deserialize(deserializer)? {
            map.insert(experiment.id.clone(), experiment);
        }
        Ok(map)
    }

    #[allow(dead_code)]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[allow(dead_code)]
    pub fn campaign_id(&self) -> &str {
        &self.campaign_id
    }

    #[allow(dead_code)]
    pub fn traffic_allocation(&self) -> &TrafficAllocation {
        &self.traffic_allocation
    }

    pub fn variation(&self, variation_id: &str) -> Option<&Variation> {
        self.variations.get(variation_id)
    }
}
