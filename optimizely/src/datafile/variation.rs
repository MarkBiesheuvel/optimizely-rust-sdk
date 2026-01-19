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
#[serde(rename_all = "camelCase")]
pub struct Variation {
    id: String,
    key: String,
    feature_enabled: bool,
}

impl Variation {
    /// Getter for `id` field
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Getter for `key` field
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Getter for `is_feature_enabled` field
    pub fn is_feature_enabled(&self) -> bool {
        self.feature_enabled
    }
}

#[derive(Debug)]
pub struct VariationMap(HashMap<String, Variation>);

impl<'de> Deserialize<'de> for VariationMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map = HashMap::new();
        for variation in Vec::<Variation>::deserialize(deserializer)? {
            map.insert(variation.id.clone(), variation);
        }

        Ok(Self(map))
    }
}

impl VariationMap {
    pub fn get(&self, id: &str) -> Option<&Variation> {
        self.0.get(id)
    }
}
