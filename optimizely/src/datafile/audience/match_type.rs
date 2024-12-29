use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub enum MatchType {
    #[serde(rename = "exists")]
    AnyValue,
    #[serde(rename = "exact")]
    Exact,
    #[serde(rename = "substring")]
    Substring,
    #[serde(rename = "lt")]
    NumberLessThan,
    #[serde(rename = "le")]
    NumberLessThanOrEqual,
    #[serde(rename = "gt")]
    NumberGreaterThan,
    #[serde(rename = "ge")]
    NumberGreaterThanOrEqual,
    #[serde(rename = "semver_eq")]
    SemVerEqual,
    #[serde(rename = "semver_lt")]
    SemVerLessThan,
    #[serde(rename = "semver_le")]
    SemVerLessThanOrEqual,
    #[serde(rename = "semver_gt")]
    SemVerGreaterThan,
    #[serde(rename = "semver_ge")]
    SemVerGreaterThanOrEqual,
}
