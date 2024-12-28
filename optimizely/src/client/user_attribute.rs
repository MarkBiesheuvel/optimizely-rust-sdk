use std::convert::From;

use crate::datafile;

/// An attribute of the user.
pub struct UserAttribute {
    #[allow(dead_code)]
    id: String,
    #[allow(dead_code)]
    key: String,
    // Unfortunately, attributes in Optimizely do not have a type.
    // Types are specified in the audience condition, however there is no guarantee that the same attribute will be compared to the same type in every audience condition.
    // The Event API expects all attributes to be a text, hence why we always store the value as a String
    #[allow(dead_code)]
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
}

impl From<(&datafile::Attribute, String)> for UserAttribute {
    /// Create user attribute by adding a value to a (datafile) attribute
    fn from((attribute, value): (&datafile::Attribute, String)) -> UserAttribute {
        UserAttribute {
            id: attribute.id().into(),
            key: attribute.key().into(),
            value: value,
        }
    }
}