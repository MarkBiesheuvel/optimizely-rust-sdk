// External imports
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Attribute {
    id: String,
    key: String,
}

impl Attribute {
    // Method to deserialize an array of Events into a Hashmap of Events
    pub fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<String, Attribute>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map = HashMap::new();
        for event in Vec::<Attribute>::deserialize(deserializer)? {
            map.insert(event.key.clone(), event);
        }
        Ok(map)
    }

    /// Getter for `id` field
    #[allow(dead_code)]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Getter for `key` field
    #[allow(dead_code)]
    pub fn key(&self) -> &str {
        &self.key
    }
}
