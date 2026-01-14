// External imports
use serde::Serialize;

// Imports from crate
use crate::client::UserAttribute;

#[derive(Serialize)]
pub struct Attribute {
    #[serde(rename = "entity_id")]
    id: String,
    key: String,
    #[serde(rename = "type")]
    attribute_type: String,
    value: String,
}

impl From<&UserAttribute> for Attribute {
    fn from(user_attribute: &UserAttribute) -> Self {
        Attribute {
            id: user_attribute.id().into(),
            key: user_attribute.key().into(),
            attribute_type: String::from("custom"),
            value: user_attribute.value().into(),
        }
    }
}
