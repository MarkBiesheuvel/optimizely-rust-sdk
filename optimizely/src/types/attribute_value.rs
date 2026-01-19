use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
#[serde(untagged)]
#[non_exhaustive]
/// The value of a user attribute
/// This supports multiple subtypes, however these are all converted to string when sending to the Event API
pub enum AttributeValue {
    /// An integer number
    Integer(u64),
    /// A decimal number
    Decimal(f64),
    /// A true or false value
    Boolean(bool),
    /// A string of text
    String(String),
    /// A non existent value
    Null,
}

// Conversion to String for Event API
impl From<&AttributeValue> for String {
    fn from(value: &AttributeValue) -> Self {
        match value {
            AttributeValue::Integer(number) => number.to_string(),
            AttributeValue::Decimal(number) => number.to_string(),
            AttributeValue::Boolean(bool) => bool.to_string(),
            AttributeValue::String(text) => text.clone(),
            AttributeValue::Null => String::new(),
        }
    }
}
