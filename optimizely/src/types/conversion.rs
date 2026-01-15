//! A conversion event

// External imports
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::{collections::HashMap, time::SystemTime};
use uuid::Uuid;

/// A conversion event
#[derive(Debug, Clone)]
pub struct Conversion {
    uuid: String,
    timestamp: u128,
    event_key: String,
    event_id: String,
    properties: HashMap<String, String>,
    tags: HashMap<String, String>,
}

impl Conversion {
    pub(crate) fn new<T: Into<String>>(
        event_key: T, event_id: T, properties: HashMap<String, String>, tags: HashMap<String, String>,
    ) -> Conversion {
        // Generate a UUID
        // This will avoid duplication when resending the same conversion to the Event API
        let uuid = Uuid::new_v4().as_hyphenated().to_string();

        // Get timestamp as milliseconds since the epoch
        let timestamp = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => duration.as_millis(),
            Err(_) => 0,
        };

        Conversion {
            uuid,
            timestamp,
            event_key: event_key.into(),
            event_id: event_id.into(),
            properties,
            tags,
        }
    }
}

impl Conversion {
    /// Get key
    pub fn event_key(&self) -> &str {
        &self.event_key
    }

    /// Get id
    pub fn event_id(&self) -> &str {
        &self.event_id
    }

    /// Get properties
    pub fn properties(&self) -> &HashMap<String, String> {
        &self.properties
    }

    /// Get tags
    pub fn tags(&self) -> &HashMap<String, String> {
        &self.tags
    }
}

impl Serialize for Conversion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("Conversion", 6)?;
        st.serialize_field("uuid", &self.uuid)?;
        st.serialize_field("timestamp", &self.timestamp)?;
        st.serialize_field("entity_id", &self.event_id)?;
        st.serialize_field("key", &self.event_key)?;
        st.serialize_field("properties", &self.properties)?;
        st.serialize_field("tags", &self.tags)?;
        st.end()
    }
}
