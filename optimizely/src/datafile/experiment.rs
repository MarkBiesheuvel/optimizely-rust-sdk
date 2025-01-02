// External imports
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

// Imports from super
use super::{TrafficAllocation, Variation, VariationMap};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Experiment {
    #[serde()]
    id: String,
    #[serde(rename = "layerId")]
    campaign_id: String,
    traffic_allocation: TrafficAllocation,
    variations: VariationMap,
}

impl Experiment {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn campaign_id(&self) -> &str {
        &self.campaign_id
    }

    pub fn traffic_allocation(&self) -> &TrafficAllocation {
        &self.traffic_allocation
    }

    pub fn variation(&self, variation_id: &str) -> Option<&Variation> {
        self.variations.get(variation_id)
    }
}

#[derive(Debug)]
pub struct ExperimentMap(HashMap<String, Experiment>);

impl<'de> Deserialize<'de> for ExperimentMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map = HashMap::new();
        for experiment in Vec::<Experiment>::deserialize(deserializer)? {
            map.insert(experiment.id.clone(), experiment);
        }

        Ok(Self(map))
    }
}

impl ExperimentMap {
    pub fn get(&self, id: &str) -> Option<&Experiment> {
        self.0.get(id)
    }
}
