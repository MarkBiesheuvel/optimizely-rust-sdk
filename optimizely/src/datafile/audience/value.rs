use semver::Version;
use serde::Deserialize;

use crate::datafile::DatafileError;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum AnyValue {
    Number(NumericValue),
    Boolean(bool),
    String(String),
    Null,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum NumericValue {
    Integer(i64),
    Decimal(f64),
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
