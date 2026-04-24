// External imports
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub(crate) struct VariableDefinition {
    id: String,
    key: String,
    #[serde(rename = "type")]
    variable_type: VariableType
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
enum VariableType {
    Boolean,
    Double,
    Integer,
    String,
}

#[derive(Debug)]
pub(crate) struct VariableDefinitionMap(HashMap<String, VariableDefinition>);

impl<'de> Deserialize<'de> for VariableDefinitionMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map = HashMap::new();
        for variable in Vec::<VariableDefinition>::deserialize(deserializer)? {
            map.insert(variable.id.clone(), variable);
        }

        Ok(Self(map))
    }
}

impl VariableDefinitionMap {
    pub fn get(&self, id: &str) -> Option<&VariableDefinition> {
        self.0.get(id)
    }
}
