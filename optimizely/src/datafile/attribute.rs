// External imports
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Attribute {
    id: String,
    key: String,
}

impl Attribute {
    /// Getter for `id` field
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Getter for `key` field
    pub fn key(&self) -> &str {
        &self.key
    }
}

#[derive(Debug)]
pub struct AttributeMap(HashMap<String, Attribute>);

impl<'de> Deserialize<'de> for AttributeMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map = HashMap::new();
        for attribute in Vec::<Attribute>::deserialize(deserializer)? {
            map.insert(attribute.key.clone(), attribute);
        }

        Ok(Self(map))
    }
}

impl AttributeMap {
    pub fn get(&self, key: &str) -> Option<&Attribute> {
        self.0.get(key)
    }
}
