use std::convert::From;

use crate::datafile;

/// An attribute of the user.
///
/// Unfortunately, attributes in Optimizely do not have a type.
/// Types are specified in the audience condition, however there is no guarantee that the same attribute will be compared to the same type in every audience condition.
/// The Event API expects all attributes to be a text, hence why we always store the value as a String
pub struct UserAttribute {
    id: String,
    key: String,
    value: String,
}

impl UserAttribute {
    /// Create user attribute by adding a value to a (datafile) attribute
    pub fn new<T: Into<String>>(id: T, key: T, value: T) -> UserAttribute {
        UserAttribute {
            id: id.into(),
            key: key.into(),
            value: value.into(),
        }
    }

    /// Get ID of the attribute
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get key of the attribute
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Get value of the attribute
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl From<(&datafile::Attribute, String)> for UserAttribute {
    /// Create user attribute by combining a value and a `datafile::Attribute`
    fn from((attribute, value): (&datafile::Attribute, String)) -> UserAttribute {
        UserAttribute {
            id: attribute.id().into(),
            key: attribute.key().into(),
            value: value,
        }
    }
}
