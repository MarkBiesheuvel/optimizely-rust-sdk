// External imports
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Event {
    id: String,
    key: String,
}

impl Event {
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
pub struct EventMap(HashMap<String, Event>);

impl<'de> Deserialize<'de> for EventMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map = HashMap::new();
        for event in Vec::<Event>::deserialize(deserializer)? {
            map.insert(event.key.clone(), event);
        }

        Ok(Self(map))
    }
}

impl EventMap {
    pub fn get(&self, key: &str) -> Option<&Event> {
        self.0.get(key)
    }
}
