use semver::Version;
use serde::Deserialize;

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

impl<'a> TryFrom<&'a str> for VersionValue {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match Version::parse(value) {
            Ok(version) => Ok(Self(version)),
            Err(_) => Err(value),
        }
    }
}
