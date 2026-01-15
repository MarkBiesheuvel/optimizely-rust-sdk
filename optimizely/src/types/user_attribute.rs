// External imports
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::fmt::Debug;

use super::AttributeValue;
use crate::datafile;

/// An attribute of the user.
///
/// The Event API expects all attributes to be a text, hence why we always store the value as a &str.
///
/// Unfortunately, references to the datafile have to be cloned in order to release the read/write lock
#[derive(Debug, Clone)]
pub struct UserAttribute {
    id: String,
    key: String,
    value: AttributeValue,
}

impl UserAttribute {
    /// Create user attribute by adding a value to a (datafile) attribute
    pub(crate) fn from_attribute_and_value(attribute: &datafile::Attribute, value: AttributeValue) -> UserAttribute {
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

impl Serialize for UserAttribute {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("UserAttribute", 4)?;
        st.serialize_field("entity_id", &self.id)?;
        st.serialize_field("key", &self.key)?;
        st.serialize_field("attribute_type", "custom")?;
        // Convert the dynamic type into a String,
        // then pass it to the "serialize_field" method which accepts a generic type
        st.serialize_field("value", &String::from(&self.value))?;
        st.end()
    }
}
