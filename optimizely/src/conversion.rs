//! A conversion event

use std::collections::HashMap;

/// A conversion event
#[derive(Debug)]
pub struct Conversion {
    event_key: String,
    event_id: String,
    properties: HashMap<String, String>,
    tags: HashMap<String, String>,
}

impl Conversion {
    pub(crate) fn new<T: Into<String>>(
        event_key: T, event_id: T, properties: HashMap<String, String>, tags: HashMap<String, String>,
    ) -> Conversion {
        Conversion {
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
