// External imports
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

use super::MatchType;

#[derive(Deserialize, Debug)]
pub enum Condition {
    AndSequence(Vec<Condition>),
    OrSequence(Vec<Condition>),
    // Negation(Condition), TODO: advanced conditions
    Match(MatchType),
}

impl Condition {
    // Method to deserialize an array of Events into a Hashmap of Events
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Condition, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Conditions are stored in an unstructered way, so parse as "Any" type
        let data = Value::deserialize(deserializer)?;

        // Recursivly parse the data
        Condition::parse(data).map_err(|msg| Error::custom(msg))
    }

    fn parse(data: Value) -> Result<Condition, &'static str> {
        // Data should be an array
        match data {
            Value::Array(array) => {
                // Parse the first item as the operator
                let operator = match array.first() {
                    Some(Value::String(operator)) => Ok(operator.to_owned()),
                    _ => Err("Condition with invalid operator type"),
                }?;

                // Parse each of the sub conditions
                let conditions = array
                    .into_iter()
                    .skip(1)
                    .map(|condition| Condition::parse(condition))
                    .collect::<Result<Vec<_>, _>>()?;

                // The operator should always be either "and" or "or"
                match operator.as_str() {
                    "and" => Ok(Condition::AndSequence(conditions)),
                    "or" => Ok(Condition::OrSequence(conditions)),
                    _ => Err("Condition with invalid operator value"),
                }
            }
            Value::Object(_) => {
                // TODO: parse using serde
                Ok(Self::Match(MatchType::AnyValue))
            },
            _ => Err("Invalid condition"),
        }
    }
}
