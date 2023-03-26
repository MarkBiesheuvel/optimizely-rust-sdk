// External imports
use error_stack::{IntoReport, Report, Result, ResultExt};
use serde_json::{from_str, Value};

// Imports from super
use super::DatafileError;

const TYPE_STRING: &str = "str";
const TYPE_INTEGER: &str = "u64";
const TYPE_BOOLEAN: &str = "bool";
const TYPE_ARRAY: &str = "array";

/// Wrapper for serde_json::Value for easier error handling
pub struct Json {
    value: Value,
}

impl Json {
    fn new(value: Value) -> Json {
        Json { value }
    }

    /// Create JSON wrapper from string
    pub fn build(content: &str) -> Result<Json, DatafileError> {
        // Parse content as JSON
        let value = from_str(content)
            .into_report()
            .change_context(DatafileError::InvalidJson)?;

        Ok(Json::new(value))
    }

    pub(crate) fn get(&mut self, key: &str) -> Result<Json, DatafileError> {
        let value = self
            .value
            .get_mut(key)
            .ok_or_else(|| {
                // Unable to find property
                Report::new(DatafileError::KeyNotFound(key.into()))
            })?
            .take();

        Ok(Json::new(value))
    }

    pub(crate) fn as_string(&self) -> Result<String, DatafileError> {
        let value = self.value.as_str().ok_or_else(|| {
            // Unable to read property as a string
            Report::new(DatafileError::InvalidType(TYPE_STRING.into()))
        })?;

        // Create owned copy
        let value = value.to_owned();

        Ok(value)
    }

    pub(crate) fn as_integer(&self) -> Result<u64, DatafileError> {
        let value = self.value.as_u64().ok_or_else(|| {
            // Unable to read property as an integer
            Report::new(DatafileError::InvalidType(TYPE_INTEGER.into()))
        })?;

        Ok(value)
    }

    pub(crate) fn as_boolean(&self) -> Result<bool, DatafileError> {
        let value = self.value.as_bool().ok_or_else(|| {
            // Unable to read property as a boolean
            Report::new(DatafileError::InvalidType(TYPE_BOOLEAN.into()))
        })?;

        Ok(value)
    }

    pub(crate) fn as_array(&mut self) -> Result<impl Iterator<Item = Json> + '_, DatafileError> {
        let iterator = self
            .value
            .as_array_mut()
            .ok_or_else(|| {
                // Unable to read property as an array
                Report::new(DatafileError::InvalidType(TYPE_ARRAY.into()))
            })?
            .iter_mut()
            .map(|value| {
                // Extract the array item from the JSON, ...
                let value = value.take();
                // ... and wrap it in a Json struct
                Json::new(value)
            });

        Ok(iterator)
    }
}
