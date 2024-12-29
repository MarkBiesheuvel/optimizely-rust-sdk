// External imports
use serde::de::{Error, IgnoredAny, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;

use super::MatchType;

const FIELD_MATCH_TYPE: &str = "match";
const FIELD_ATTRIBUTE_NAME: &str = "name";

#[derive(Debug, PartialEq)]
pub enum Condition {
    AndSequence(Vec<Condition>),
    OrSequence(Vec<Condition>),
    Match {
        match_type: MatchType,
        attribute_name: String,
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
                "value" => {
                    // TODO: implement
                    map.next_value::<IgnoredAny>()?;
                }
                _ => {
                    // Skip unknown fields
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }

        let match_type = match_type.ok_or_else(|| Error::missing_field(FIELD_MATCH_TYPE))?;
        let attribute_name = attribute_name.ok_or_else(|| Error::missing_field(FIELD_ATTRIBUTE_NAME))?;

        Ok(Condition::Match {
            match_type,
            attribute_name,
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

    #[test]
    fn single_match() {
        let json = r#"{"match":"semver_ge","name":"app_version","type":"custom_attribute","value":"0.4.0"}"#;

        assert_eq!(
            serde_json::from_str::<Condition>(json).unwrap(),
            Condition::Match {
                match_type: MatchType::SemVerGreaterThanOrEqual,
                attribute_name: String::from("app_version")
            }
        );
    }
}
