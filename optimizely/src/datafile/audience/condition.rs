// External imports
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

use super::MatchType;

#[derive(Deserialize, Debug)]
pub enum Condition {
    #[allow(dead_code)]
    AndCondition(Vec<Condition>),
    #[allow(dead_code)]
    OrCondition(Vec<Condition>),
    #[allow(dead_code)]
    OrMatch(Vec<MatchType>),
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
        let data = match data {
            Value::Array(array) => Ok(array),
            _ => Err("Invalid condition"),
        }?;

        // Get reference to first two items
        let first_item = data.get(0);
        let second_item = data.get(1);

        // Parse the first item as the operator
        let operator = match first_item {
            Some(Value::String(operator)) => Ok(operator),
            _ => Err("Condition with invalid operator type"),
        }?;

        // The operator should always be either "and" or "or"
        match operator.as_str() {
            "and" => {
                // Parse each of the sub conditions
                let conditions = data
                    .into_iter()
                    .skip(1)
                    .map(|condition| Condition::parse(condition))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(Condition::AndCondition(conditions))
            }
            "or" => match second_item {
                Some(Value::Array(_)) => {
                    // Parse each of the sub conditions
                    let conditions = data
                        .into_iter()
                        .skip(1)
                        .map(|condition| Condition::parse(condition))
                        .collect::<Result<Vec<_>, _>>()?;

                    Ok(Condition::OrCondition(conditions))
                }
                Some(Value::Object(_)) => {
                    // Parse each of the matches
                    let matches = data
                        .into_iter()
                        .skip(1)
                        .map(|_| MatchType::AnyValue)
                        .collect();

                    Ok(Condition::OrMatch(matches))
                }
                v => {
                    dbg!(v);
                    Err("Unexpected type in condition")
                }
            },
            _ => Err("Condition with invalid operator value"),
        }
    }
}
