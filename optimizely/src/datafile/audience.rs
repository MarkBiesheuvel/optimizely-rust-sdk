// External imports
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

use condition::Condition;
use match_type::MatchType;

mod condition;
mod match_type;

#[derive(Deserialize, Debug)]
pub struct Audience {
    id: String,
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    conditions: Condition,
}

impl Audience {
    // Method to deserialize an array of Events into a Hashmap of Events
    pub fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<String, Audience>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map = HashMap::new();
        for audience in Vec::<Audience>::deserialize(deserializer)? {
            map.insert(audience.id.clone(), audience);
        }

        dbg!(map);
        panic!();
        #[allow(unreachable_code)]
        Ok(map)
    }
}
