// External imports
use error_stack::Result;

// Imports from super
use super::{Context, DatafileError};

/// A single variation like "off", "on" or other user-created variations.
///
/// A variation has the properties `id`, `key`, and `is_feature_enabled`.
/// The `id` is a unique identifier.
/// The `key` is a human-readable value.
/// The value of `is_feature_enabled` is `false` for the "off" variation.
/// All other variations will have `is_feature_enabled` is `true`.
#[derive(Debug, PartialEq, Eq)]
pub struct Variation {
    id: String,
    key: String,
    is_feature_enabled: bool,
}

impl Variation {
    // Create a new variation
    pub(crate) fn new<T: Into<String>>(id: T, key: T, is_feature_enabled: bool) -> Variation {
        Variation {
            id: id.into(),
            key: key.into(),
            is_feature_enabled,
        }
    }

    /// Create a new variation from Context
    pub(crate) fn build(context: &mut Context) -> Result<Variation, DatafileError> {
        // Get variation_id as String
        let id = context.get("id")?.as_string()?;

        // Get variation_key as String
        let key = context.get("key")?.as_string()?;

        // TODO: fix bug below again
        // BUG: Found an example datafile where this field is missing, therefore default to `false`
        let is_feature_enabled = context.get("featureEnabled")?.as_boolean()?;

        Ok(Variation::new(id, key, is_feature_enabled))
    }

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
        self.is_feature_enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_simple_variation() {
        let variation = Variation::new("1234", "on", true);

        assert!(variation.is_feature_enabled());
        assert_eq!(variation.key(), "on");
    }
}
