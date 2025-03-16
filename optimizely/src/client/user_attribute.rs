use crate::datafile;
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

/// An attribute of the user.
///
/// Unfortunately, attributes in Optimizely do not have a type.
/// Types are specified in the audience condition, however there is no guarantee that the same attribute will be
/// compared to the same type in every audience condition.
/// The Event API expects all attributes to be a text, hence why we always store the value as a &str.
/// 
/// Unfortunately, references to the datafile have to be cloned in order to release the read/write lock
#[derive(Debug)]
pub struct UserAttribute<'a> {
    id: String,
    key: String,
    value: &'a str,
}

impl UserAttribute<'_> {
    /// Create user attribute by adding a value to a (datafile) attribute
    pub fn from_attribute_and_value<'a>(attribute: &datafile::Attribute, value: &'a str) -> UserAttribute<'a> {
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
    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Debug, Default)]
/// Mapping of attribute key to UserAttribute
/// TODO: rewrite to map from datafile::Attribute to value
pub struct UserAttributeMap<'a>(HashMap<String, UserAttribute<'a>>);

impl<'a> Deref for UserAttributeMap<'a> {
    type Target = HashMap<String, UserAttribute<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for UserAttributeMap<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
