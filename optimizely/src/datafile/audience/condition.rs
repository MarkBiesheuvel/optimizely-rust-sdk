// External imports
use serde::de::{Error, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;

use super::match_type::MatchType;
use super::operator::{NumericOperator, StringOperator};
use super::value::{AnyValue, NumericValue};

#[derive(Deserialize, Debug)]
enum Field {
    #[serde(rename = "match")]
    MatchType,
    #[serde(rename = "name")]
    AttributeName,
    #[serde(rename = "value")]
    Value,
    #[serde(rename = "type")]
    Type,
}

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
    StringComparison {
        attribute_name: AttributeName,
        operator: StringOperator,
        value: String,
    },
    BooleanComparison {
        attribute_name: AttributeName,
        value: bool,
    },
    Exists {
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

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        // Start with all variables set to none
        let mut match_type = Option::None;
        let mut attribute_name = Option::None;
        let mut value = Option::None;

        // Iterate ovr all keys
        while let Some(key) = map.next_key::<Field>()? {
            match key {
                Field::MatchType => {
                    if match_type.is_some() {
                        return Err(Error::duplicate_field("match"));
                    }
                    match_type = Some(map.next_value::<MatchType>()?);
                }
                Field::AttributeName => {
                    if attribute_name.is_some() {
                        return Err(Error::duplicate_field("name"));
                    }
                    attribute_name = Some(map.next_value::<AttributeName>()?);
                }
                Field::Value => {
                    if value.is_some() {
                        return Err(Error::duplicate_field("value"));
                    }
                    value = Some(map.next_value::<AnyValue>()?);
                }
                Field::Type => {
                    // Skip type field as it is always "custom_attribute"
                    let _type = map.next_value::<String>()?;
                    assert_eq!(_type, "custom_attribute");
                }
            }
        }

        // Verify that match type and attribute name have been set
        let match_type = match_type.ok_or_else(|| Error::missing_field("match"))?;
        let attribute_name = attribute_name.ok_or_else(|| Error::missing_field("name"))?;

        // Value is optional. It is not needed for exists
        let value = value.unwrap_or_else(|| AnyValue::Null);

        // Only accept valid combinations of match type and value type
        let condition = match value {
            // Checking whether an attribute exists
            AnyValue::Null => {
                // Only one valid operator
                if match_type != MatchType::Exists {
                    return Err(Error::custom("invalid operator for empty type"));
                }

                Condition::Exists { attribute_name }
            }
            // Comparing an attribute to a boolean value
            AnyValue::Boolean(value) => {
                // Only one valid operator
                if match_type != MatchType::Exact {
                    return Err(Error::custom("invalid operator for boolean"));
                }

                Condition::BooleanComparison {
                    attribute_name,
                    value,
                }
            }
            // Comparing an attribute to a numeric value
            AnyValue::Number(value) => {
                let operator = match match_type {
                    MatchType::Exact => NumericOperator::Equal,
                    MatchType::LessThan => NumericOperator::LessThan,
                    MatchType::LessThanOrEqual => NumericOperator::LessThanOrEqual,
                    MatchType::GreaterThan => NumericOperator::GreaterThan,
                    MatchType::GreaterThanOrEqual => NumericOperator::GreaterThanOrEqual,
                    _ => return Err(Error::custom("invalid operator for number")),
                };

                Condition::NumericComparison {
                    operator,
                    attribute_name,
                    value,
                }
            }
            // Comparing an attribute to a string value
            AnyValue::String(value) => {
                let operator = match match_type {
                    MatchType::Exact => StringOperator::Equal,
                    MatchType::Substring => StringOperator::Contains,
                    MatchType::SemVerEqual => StringOperator::SemVerEqual,
                    MatchType::SemVerLessThan => StringOperator::SemVerLessThan,
                    MatchType::SemVerLessThanOrEqual => StringOperator::SemVerLessThanOrEqual,
                    MatchType::SemVerGreaterThan => StringOperator::SemVerGreaterThan,
                    MatchType::SemVerGreaterThanOrEqual => StringOperator::SemVerGreaterThanOrEqual,
                    _ => {
                        return Err(Error::custom("invalid operator for string"));
                    }
                };

                Condition::StringComparison {
                    operator,
                    attribute_name,
                    value,
                }
            }
        };

        Ok(condition)
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

        let expected = Condition::StringComparison {
            attribute_name: String::from("app_version"),
            operator: StringOperator::SemVerGreaterThanOrEqual,
            value: String::from("0.4.0"),
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
