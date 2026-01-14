use super::match_type::MatchType;
use super::operator::{NumericOperator, SemVerOperator, StringOperator};
use crate::client::UserAttributeMap;
use crate::AttributeValue;
use semver::Version;
use serde::de::{Error, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
// A simplified version of a condition to simplify parsing
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
    Negation(Box<Condition>),
    IntegerComparison {
        attribute_name: AttributeName,
        operator: NumericOperator,
        desired_value: u64,
    },
    DecimalComparison {
        attribute_name: AttributeName,
        operator: NumericOperator,
        desired_value: f64,
    },
    StringComparison {
        attribute_name: AttributeName,
        operator: StringOperator,
        desired_value: String,
    },
    BooleanComparison {
        attribute_name: AttributeName,
        desired_value: bool,
    },
    Exists {
        attribute_name: AttributeName,
    },
}

impl Condition {
    /// Whether the user attributes match the condition or not
    pub fn does_match(&self, user_attributes: &UserAttributeMap) -> bool {
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
            Condition::Negation(condition) => {
                // Negate the result of condition within
                !condition.does_match(user_attributes)
            },
            Condition::Exists { attribute_name } => {
                // Verify that attribute does exist
                user_attributes.get(attribute_name).is_some()
            }
            Condition::BooleanComparison {
                attribute_name,
                desired_value,
            } => {
                // Retrieve value
                user_attributes
                    .get(attribute_name)
                    .map(|user_attribute| {
                        // Instead of parsing a string to bool, we'll just match cases
                        match user_attribute.value() {
                            // User has attribute set to true, so the condition is true if the desired value is true
                            AttributeValue::Boolean(true) => *desired_value,
                            // User has attribute set to false, so the condition is true if the desired value is false
                            AttributeValue::Boolean(false) => !desired_value,
                            // Not a valid bool, so does not match
                            _ => false,
                        }
                    })
                    .unwrap_or(false)
            }
            Condition::StringComparison {
                attribute_name,
                operator,
                desired_value,
            } => {
                // Retrieve value
                user_attributes
                    .get(attribute_name)
                    .map(|user_attribute| {
                        let user_attribute_value = match user_attribute.value() {
                            AttributeValue::String(value) => value,
                            _ => {
                                // Cannot perform StringComparison on a non String value
                                return false;
                            }
                        };

                        // Apply string operator
                        match operator {
                            StringOperator::Equal => desired_value == user_attribute_value,
                            StringOperator::Contains => user_attribute_value.contains(desired_value),
                            StringOperator::SemVer(sem_ver_operator) => {
                                let user_attribute_value = match Version::parse(user_attribute_value) {
                                    Ok(version) => version,
                                    Err(_) => {
                                        // Unable to parse String as version number
                                        return false;
                                    }
                                };
                                let desired_value = match Version::parse(desired_value) {
                                    Ok(version) => version,
                                    Err(_) => {
                                        // Unable to parse String as version number
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
            Condition::IntegerComparison {
                attribute_name,
                operator,
                desired_value,
            } => {
                // Retrieve value
                user_attributes
                    .get(attribute_name)
                    .map(|user_attribute| {
                        let user_attribute_value = match user_attribute.value() {
                            AttributeValue::Integer(value) => value,
                            _ => {
                                // Cannot perform IntegerComparison on a non Integer value
                                return false;
                            }
                        };
                        // Apply operator
                        match operator {
                            NumericOperator::Equal => user_attribute_value == desired_value,
                            NumericOperator::LessThan => user_attribute_value < desired_value,
                            NumericOperator::LessThanOrEqual => user_attribute_value <= desired_value,
                            NumericOperator::GreaterThan => user_attribute_value > desired_value,
                            NumericOperator::GreaterThanOrEqual => user_attribute_value >= desired_value,
                        }
                    })
                    .unwrap_or(false)
            }
            Condition::DecimalComparison {
                attribute_name,
                operator,
                desired_value,
            } => {
                // Retrieve value
                user_attributes
                    .get(attribute_name)
                    .map(|user_attribute| {
                        let user_attribute_value = match user_attribute.value() {
                            AttributeValue::Decimal(value) => value,
                            _ => {
                                // Cannot perform DecimalComparison on a non Decimal value
                                return false;
                            }
                        };
                        // Apply operator
                        match operator {
                            NumericOperator::Equal => user_attribute_value == desired_value,
                            NumericOperator::LessThan => user_attribute_value < desired_value,
                            NumericOperator::LessThanOrEqual => user_attribute_value <= desired_value,
                            NumericOperator::GreaterThan => user_attribute_value > desired_value,
                            NumericOperator::GreaterThanOrEqual => user_attribute_value >= desired_value,
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
            "not" => {
                if conditions.len() > 1 {
                    return Err(Error::custom("too many conditions found within not statement"));
                }
                let condition = match conditions.pop() {
                    Some(condition) =>  condition,
                    None => {
                        return Err(Error::custom("no condition found within not statement"));
                    }
                };
                Condition::Negation(Box::new(condition))
            }
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
                    value = Some(map.next_value::<AttributeValue>()?);
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
        let value = value.unwrap_or(AttributeValue::Null);

        // Only accept valid combinations of match type and value type
        let condition = match value {
            // Checking whether an attribute exists
            AttributeValue::Null => {
                // Only one valid operator
                if match_type != MatchType::Exists {
                    return Err(Error::custom("invalid operator for empty type"));
                }

                Condition::Exists { attribute_name }
            }
            // Comparing an attribute to a boolean value
            AttributeValue::Boolean(desired_value) => {
                // Only one valid operator
                if match_type != MatchType::Exact {
                    return Err(Error::custom("invalid operator for boolean"));
                }

                Condition::BooleanComparison {
                    attribute_name,
                    desired_value,
                }
            }
            // Comparing an attribute to a numeric value
            AttributeValue::Integer(desired_value) => {
                let operator = match match_type {
                    MatchType::Exact => NumericOperator::Equal,
                    MatchType::LessThan => NumericOperator::LessThan,
                    MatchType::LessThanOrEqual => NumericOperator::LessThanOrEqual,
                    MatchType::GreaterThan => NumericOperator::GreaterThan,
                    MatchType::GreaterThanOrEqual => NumericOperator::GreaterThanOrEqual,
                    _ => return Err(Error::custom("invalid operator for number")),
                };

                Condition::IntegerComparison {
                    operator,
                    attribute_name,
                    desired_value,
                }
            }
            // Comparing an attribute to a numeric value
            AttributeValue::Decimal(desired_value) => {
                let operator = match match_type {
                    MatchType::Exact => NumericOperator::Equal,
                    MatchType::LessThan => NumericOperator::LessThan,
                    MatchType::LessThanOrEqual => NumericOperator::LessThanOrEqual,
                    MatchType::GreaterThan => NumericOperator::GreaterThan,
                    MatchType::GreaterThanOrEqual => NumericOperator::GreaterThanOrEqual,
                    _ => return Err(Error::custom("invalid operator for number")),
                };

                Condition::DecimalComparison {
                    operator,
                    attribute_name,
                    desired_value,
                }
            }
            // Comparing an attribute to a string value
            AttributeValue::String(desired_value) => {
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
                    desired_value,
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
            desired_value: String::from("0.4.0"),
        };

        // Parse successfully
        assert_eq!(serde_json::from_str::<Condition>(json)?, condition);

        // TODO: check against user attribute value

        Ok(())
    }

    #[test]
    fn structured_sequence() -> Result<(), Box<dyn Error>> {
        let json = r#"["and",["or",["or",{"match":"substring","name":"currentPath","type":"custom_attribute","value":"/checkout"}]]]"#;

        // First layer, AND-sequence
        let expected = Condition::AndSequence(Vec::from([
            // Second layer, OR-sequence
            Condition::OrSequence(Vec::from([
                // Third layer, OR-sequence
                Condition::OrSequence(Vec::from([
                    // Fourth layer, match
                    Condition::StringComparison {
                        attribute_name: String::from("currentPath"),
                        operator: StringOperator::Contains,
                        desired_value: String::from("/checkout"),
                    },
                ])),
            ])),
        ]));

        assert_eq!(serde_json::from_str::<Condition>(json)?, expected);

        Ok(())
    }
}
