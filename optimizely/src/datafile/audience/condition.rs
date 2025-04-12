use super::match_type::MatchType;
use super::operator::{NumericOperator, SemVerOperator, StringOperator};
use super::value::{AnyValue, NumericValue};
use crate::client::UserAttributeMap;
use semver::Version;
use serde::de::{Error, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
enum Field {
    #[serde(rename = "match")]
    MatchType,
    #[serde(rename = "name")]
    AttributeName,
    Value,
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

impl Condition {
    /// Whether the user attributes match the condition or not
    pub fn does_match(&self, user_attributes: &UserAttributeMap<'_>) -> bool {
        match self {
            Condition::AndSequence(sequence) => {
                // Combine sequence with AND
                sequence
                    .iter()
                    .all(|condition| condition.does_match(user_attributes))
            }
            Condition::OrSequence(sequence) => {
                // Combine sequence with OR
                sequence
                    .iter()
                    .any(|condition| condition.does_match(user_attributes))
            }
            Condition::Exists { attribute_name } => {
                // Verify that attribute does exist
                user_attributes.get(attribute_name).is_some()
            }
            Condition::BooleanComparison {
                attribute_name,
                value,
            } => {
                // Retrieve value
                user_attributes
                    .get(attribute_name)
                    .map(|user_attribute| {
                        // Instead of parsing a string to bool, we'll just match cases
                        match user_attribute.value() {
                            // User has attribute set to true, so the condition is true if the desired value is true
                            "true" => *value,
                            // User has attribute set to false, so the condition is true if the desired value is false
                            "false" => !value,
                            // Not a valid bool, so does not match
                            _ => false,
                        }
                    })
                    .unwrap_or(false)
            }
            Condition::StringComparison {
                attribute_name,
                operator,
                value,
            } => {
                // Retrieve value
                user_attributes
                    .get(attribute_name)
                    .map(|user_attribute| {
                        let user_attribute_value = user_attribute.value();

                        // Apply string operator
                        match operator {
                            StringOperator::Equal => value == user_attribute_value,
                            StringOperator::Contains => user_attribute_value.contains(value),
                            StringOperator::SemVer(sem_ver_operator) => {
                                let user_attribute_value = match Version::parse(user_attribute_value) {
                                    Ok(version) => version,
                                    Err(_) => {
                                        return false;
                                    }
                                };
                                let desired_value = match Version::parse(value) {
                                    Ok(version) => version,
                                    Err(_) => {
                                        return false;
                                    }
                                };
                                // Apply semantic version operator
                                match sem_ver_operator {
                                    SemVerOperator::Equal => user_attribute_value == desired_value,
                                    SemVerOperator::LessThan => user_attribute_value < desired_value,
                                    SemVerOperator::LessThanOrEqual => user_attribute_value <= desired_value,
                                    SemVerOperator::GreaterThan => user_attribute_value > desired_value,
                                    SemVerOperator::GreaterThanOrEqual => user_attribute_value >= desired_value,
                                }
                            }
                        }
                    })
                    .unwrap_or(false)
            }
            Condition::NumericComparison {
                attribute_name,
                operator,
                value: _,
            } => {
                // Retrieve value
                user_attributes
                    .get(attribute_name)
                    .map(|user_attribute| {
                        let _user_attribute_value = user_attribute.value();

                        // TODO: compare user_attribute_value string to NumericValue enum

                        // Apply operator
                        match operator {
                            NumericOperator::Equal => todo!(),
                            NumericOperator::LessThan => todo!(),
                            NumericOperator::LessThanOrEqual => todo!(),
                            NumericOperator::GreaterThan => todo!(),
                            NumericOperator::GreaterThanOrEqual => todo!(),
                        }
                    })
                    .unwrap_or(false)
            }
        }
    }
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

        let condition = match operator.as_str() {
            "and" => Condition::AndSequence(conditions),
            "or" => Condition::OrSequence(conditions),
            _ => {
                return Err(Error::custom(r#"expected either "and" or "or""#));
            }
        };

        Ok(condition)
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
        let value = value.unwrap_or(AnyValue::Null);

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
                    MatchType::SemVerEqual => StringOperator::SemVer(SemVerOperator::Equal),
                    MatchType::SemVerLessThan => StringOperator::SemVer(SemVerOperator::LessThan),
                    MatchType::SemVerLessThanOrEqual => StringOperator::SemVer(SemVerOperator::LessThanOrEqual),
                    MatchType::SemVerGreaterThan => StringOperator::SemVer(SemVerOperator::GreaterThan),
                    MatchType::SemVerGreaterThanOrEqual => StringOperator::SemVer(SemVerOperator::GreaterThanOrEqual),
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
        // JSON encoded condition
        let json = r#"{"match":"semver_ge","name":"app_version","type":"custom_attribute","value":"0.4.0"}"#;

        // Native condition
        let condition = Condition::StringComparison {
            attribute_name: String::from("app_version"),
            operator: StringOperator::SemVer(SemVerOperator::GreaterThanOrEqual),
            value: String::from("0.4.0"),
        };

        // Parse successfully
        assert_eq!(serde_json::from_str::<Condition>(json)?, condition);

        // TODO: check against user attribute value

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
