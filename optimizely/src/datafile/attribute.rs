// External imports
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Attribute {
    id: String,
    key: String,
}

impl Attribute {
    // Method to deserialize an array of Attributes into a Hashmap of Attributes
    pub fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<String, Attribute>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map = HashMap::new();
        for attribute in Vec::<Attribute>::deserialize(deserializer)? {
            map.insert(attribute.key.clone(), attribute);
        }
        Ok(map)
    }

    /// Getter for `id` field
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Getter for `key` field
    pub fn key(&self) -> &str {
        &self.key
    }
}
