use semver::Version;
use serde::Deserialize;

use crate::datafile::DatafileError;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum NumericValue {
    Integer(i64),
    Decimal(f64),
}

impl TryFrom<serde_json::Number> for NumericValue {
    type Error = DatafileError;

    fn try_from(value: serde_json::Number) -> Result<Self, Self::Error> {
        if let Some(integer) = value.as_i64() {
            Ok(Self::Integer(integer))
        } else if let Some(float) = value.as_f64() {
            Ok(Self::Decimal(float))
        } else {
            Err(DatafileError::InvalidJson)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct VersionValue(Version);

impl TryFrom<&str> for VersionValue {
    type Error = DatafileError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match Version::parse(value) {
            Ok(version) => Ok(Self(version)),
            Err(_) => Err(DatafileError::InvalidJson),
        }
    }
}
