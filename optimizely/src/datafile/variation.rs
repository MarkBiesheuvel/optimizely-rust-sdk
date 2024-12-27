// External imports
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

/// A single variation like "off", "on" or other user-created variations.
///
/// A variation has the properties `id`, `key`, and `is_feature_enabled`.
/// The `id` is a unique identifier.
/// The `key` is a human-readable value.
/// The value of `is_feature_enabled` is `false` for the "off" variation.
/// All other variations will have `is_feature_enabled` is `true`.
#[derive(Debug, Deserialize)]
pub struct Variation {
    #[serde()]
    id: String,
    #[serde()]
    key: String,
    #[serde(rename = "featureEnabled")]
    is_feature_enabled: bool,
}

impl Variation {
    // Method to deserialize an array of Variations into a Hashmap of Variations
    pub fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<String, Variation>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map = HashMap::new();
        for variation in Vec::<Variation>::deserialize(deserializer)? {
            map.insert(variation.id.clone(), variation);
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

    /// Getter for `is_feature_enabled` field
    #[allow(dead_code)]
    pub fn is_feature_enabled(&self) -> bool {
        self.is_feature_enabled
    }
}
