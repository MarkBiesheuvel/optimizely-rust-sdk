use crate::datafile;
use crate::AttributeValue;
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

/// An attribute of the user.
///
/// The Event API expects all attributes to be a text, hence why we always store the value as a &str.
///
/// Unfortunately, references to the datafile have to be cloned in order to release the read/write lock
#[derive(Debug)]
pub struct UserAttribute {
    id: String,
    key: String,
    value: AttributeValue,
}

impl UserAttribute {
    /// Create user attribute by adding a value to a (datafile) attribute
    pub fn from_attribute_and_value(attribute: &datafile::Attribute, value: AttributeValue) -> UserAttribute {
        UserAttribute {
            id: attribute.id().into(),
            key: attribute.key().into(),
            value,
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
    pub fn value(&self) -> &AttributeValue {
        &self.value
    }
}

#[derive(Debug, Default)]
/// Mapping of attribute key to UserAttribute
/// TODO: rewrite to map from datafile::Attribute to value
pub struct UserAttributeMap(HashMap<String, UserAttribute>);

impl<'a> Deref for UserAttributeMap {
    type Target = HashMap<String, UserAttribute>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for UserAttributeMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
