// External imports
use serde::de::{Error, IgnoredAny, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;

use super::operator::{NumericOperator, StringOperator, VersionOperator};
use super::value::{AnyValue, NumericValue, VersionValue};

const FIELD_MATCH_TYPE: &str = "match";
const FIELD_ATTRIBUTE_NAME: &str = "name";
const FIELD_VALUE: &str = "value";

type AttributeName = String;

#[derive(Debug, PartialEq)]
pub enum Condition {
    AndSequence(Vec<Condition>),
    OrSequence(Vec<Condition>),
    NumericComparison {
        attribute_name: AttributeName,
        operator: NumericOperator,
        value: NumericValue,
    },
    VersionComparison {
        attribute_name: AttributeName,
        operator: VersionOperator,
        value: VersionValue,
    },
    StringComparison {
        attribute_name: AttributeName,
        operator: StringOperator,
        value: String,
    },
    BooleanComparison {
        attribute_name: AttributeName,
        value: bool,
    },
    AnyValue {
        attribute_name: AttributeName,
    },
}

// Advanced serde Deserialize
struct ConditionVisitor;
impl<'de> Visitor<'de> for ConditionVisitor {
    type Value = Condition;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a sequence or map")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let operator = seq
            .next_element::<String>()?
            .ok_or_else(|| Error::custom("expected at least one element"))?;

        let mut conditions = Vec::new();

        while let Some(condition) = seq.next_element::<Condition>()? {
            conditions.push(condition);
        }

        match operator.as_str() {
            "and" => Ok(Condition::AndSequence(conditions)),
            "or" => Ok(Condition::OrSequence(conditions)),
            _ => Err(Error::custom(r#"expected either "and" or "or""#)),
        }
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut match_type = Option::<String>::None;
        let mut attribute_name = Option::<String>::None;
        let mut value = Option::<AnyValue>::None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                FIELD_MATCH_TYPE => {
                    if match_type.is_some() {
                        return Err(Error::duplicate_field(FIELD_MATCH_TYPE));
                    }
                    match_type = Some(map.next_value()?);
                }
                FIELD_ATTRIBUTE_NAME => {
                    if attribute_name.is_some() {
                        return Err(Error::duplicate_field(FIELD_ATTRIBUTE_NAME));
                    }
                    attribute_name = Some(map.next_value()?);
                }
                FIELD_VALUE => {
                    if value.is_some() {
                        return Err(Error::duplicate_field(FIELD_VALUE));
                    }
                    value = Some(map.next_value()?);
                }
                _ => {
                    // Skip unknown fields
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }

        let match_type = match_type.ok_or_else(|| Error::missing_field(FIELD_MATCH_TYPE))?;
        let attribute_name = attribute_name.ok_or_else(|| Error::missing_field(FIELD_ATTRIBUTE_NAME))?;
        let value = value.ok_or_else(|| Error::missing_field(FIELD_VALUE))?;

        match (match_type.as_str(), value) {
            // Checking whether an attribute has any value
            ("exists", AnyValue::Null) => Ok(Condition::AnyValue { attribute_name }),
            // Checking whether an attribute is equal to a string value
            ("exact", AnyValue::String(value)) => Ok(Condition::StringComparison {
                operator: StringOperator::Equal,
                attribute_name,
                value,
            }),
            // Checking whether an attribute contains a string value
            ("substring", AnyValue::String(value)) => Ok(Condition::StringComparison {
                operator: StringOperator::Contains,
                attribute_name,
                value,
            }),
            // Checking whether an attribute is equal to a bool value
            ("exact", AnyValue::Boolean(value)) => Ok(Condition::BooleanComparison {
                attribute_name,
                value,
            }),
            // Checking whether an attribute is equal to a numeric value
            ("exact", AnyValue::Number(value)) => Ok(Condition::NumericComparison {
                operator: NumericOperator::Equal,
                attribute_name,
                value,
            }),
            // Checking whether an attribute is less than a numeric value
            ("lt", AnyValue::Number(value)) => Ok(Condition::NumericComparison {
                operator: NumericOperator::LessThan,
                attribute_name,
                value,
            }),
            // Checking whether an attribute is less than or equal to a numeric value
            ("le", AnyValue::Number(value)) => Ok(Condition::NumericComparison {
                operator: NumericOperator::LessThanOrEqual,
                attribute_name,
                value,
            }),
            // Checking whether an attribute is greater than a numeric value
            ("gt", AnyValue::Number(value)) => Ok(Condition::NumericComparison {
                operator: NumericOperator::GreaterThan,
                attribute_name,
                value,
            }),
            // Checking whether an attribute is greater than or equal to a numeric value
            ("ge", AnyValue::Number(value)) => Ok(Condition::NumericComparison {
                operator: NumericOperator::GreaterThanOrEqual,
                attribute_name,
                value,
            }),
            // Checking whether an attribute is equal to a version number
            ("semver_eq", AnyValue::String(value)) => Ok(Condition::VersionComparison {
                operator: VersionOperator::Equal,
                attribute_name,
                value: VersionValue::try_from(&*value).unwrap(),
            }),
            // Checking whether an attribute is less than a version number
            ("semver_lt", AnyValue::String(value)) => Ok(Condition::VersionComparison {
                operator: VersionOperator::LessThan,
                attribute_name,
                value: VersionValue::try_from(&*value).unwrap(),
            }),
            // Checking whether an attribute is less than or equal to a version number
            ("semver_le", AnyValue::String(value)) => Ok(Condition::VersionComparison {
                operator: VersionOperator::LessThanOrEqual,
                attribute_name,
                value: VersionValue::try_from(&*value).unwrap(),
            }),
            // Checking whether an attribute is greater than a version number
            ("semver_gt", AnyValue::String(value)) => Ok(Condition::VersionComparison {
                operator: VersionOperator::GreaterThan,
                attribute_name,
                value: VersionValue::try_from(&*value).unwrap(),
            }),
            // Checking whether an attribute is greater than or equal to a version number
            ("semver_ge", AnyValue::String(value)) => Ok(Condition::VersionComparison {
                operator: VersionOperator::GreaterThanOrEqual,
                attribute_name,
                value: VersionValue::try_from(&*value).unwrap(),
            }),
            _ => Err(Error::custom("invalid configuration of condition")),
        }
    }
}

impl<'de> Deserialize<'de> for Condition {
    fn deserialize<D>(deserializer: D) -> Result<Condition, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ConditionVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn single_match() -> Result<(), Box<dyn Error>> {
        let json = r#"{"match":"semver_ge","name":"app_version","type":"custom_attribute","value":"0.4.0"}"#;

        let expected = Condition::VersionComparison {
            attribute_name: String::from("app_version"),
            operator: VersionOperator::GreaterThanOrEqual,
            value: VersionValue::try_from("0.4.0")?,
        };

        assert_eq!(serde_json::from_str::<Condition>(json)?, expected);

        Ok(())
    }

    #[test]
    fn structured_sequence() -> Result<(), Box<dyn Error>> {
        let json = r#"["and",["or",["or",{"match":"substring","name":"currentUri","type":"custom_attribute","value":"/checkout"}]]]"#;

        // First layer, AND-sequence
        let expected = Condition::AndSequence(Vec::from([
            // Second layer, OR-sequence
            Condition::OrSequence(Vec::from([
                // Third layer, OR-sequence
                Condition::OrSequence(Vec::from([
                    // Fourth layer, match
                    Condition::StringComparison {
                        attribute_name: String::from("currentUri"),
                        operator: StringOperator::Contains,
                        value: String::from("/checkout"),
                    },
                ])),
            ])),
        ]));

        assert_eq!(serde_json::from_str::<Condition>(json)?, expected);

        Ok(())
    }
}
