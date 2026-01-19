// External imports
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

use condition::Condition;

mod condition;
mod match_type;
mod operator;

#[derive(Deserialize, Debug)]
pub(crate) struct Audience {
    id: String,
    #[allow(dead_code)]
    name: String,
    #[serde(rename = "conditions")]
    condition: Condition,
}

impl Audience {
    /// Return the condition of this audience
    pub fn condition(&self) -> &Condition {
        &self.condition
    }
}

#[derive(Debug)]
pub struct AudienceMap(HashMap<String, Audience>);

impl<'de> Deserialize<'de> for AudienceMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map = HashMap::new();
        for audience in Vec::<Audience>::deserialize(deserializer)? {
            map.insert(audience.id.clone(), audience);
        }

        Ok(Self(map))
    }
}

impl AudienceMap {
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&Audience> {
        self.0.get(key)
    }
}
