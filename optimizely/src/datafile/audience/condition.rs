// External imports
use serde::de::{Error, IgnoredAny, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;

use super::{MatchType, Value};

const FIELD_MATCH_TYPE: &str = "match";
const FIELD_ATTRIBUTE_NAME: &str = "name";
const FIELD_VALUE: &str = "value";

#[derive(Debug, PartialEq)]
pub enum Condition {
    AndSequence(Vec<Condition>),
    OrSequence(Vec<Condition>),
    Match {
        match_type: MatchType,
        attribute_name: String,
        value: Value,
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
        let mut match_type = None;
        let mut attribute_name = None;
        let mut value = None;

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

        // NOTE: some combinations of MatchType and value are invalid

        Ok(Condition::Match {
            match_type,
            attribute_name,
            value,
        })
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

        let expected = Condition::Match {
            match_type: MatchType::SemVerGreaterThanOrEqual,
            attribute_name: String::from("app_version"),
            value: Value::String(String::from("0.4.0")),
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
                    Condition::Match {
                        match_type: MatchType::Substring,
                        attribute_name: String::from("currentUri"),
                        value: Value::String(String::from("/checkout")),
                    },
                ])),
            ])),
        ]));

        assert_eq!(serde_json::from_str::<Condition>(json)?, expected);

        Ok(())
    }
}
