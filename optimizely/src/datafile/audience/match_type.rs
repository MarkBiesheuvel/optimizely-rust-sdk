use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub enum MatchType {
    #[serde(rename = "exists")]
    Exists,
    #[serde(rename = "exact")]
    Exact,
    #[serde(rename = "substring")]
    Substring,
    #[serde(rename = "lt")]
    LessThan,
    #[serde(rename = "le")]
    LessThanOrEqual,
    #[serde(rename = "gt")]
    GreaterThan,
    #[serde(rename = "ge")]
    GreaterThanOrEqual,
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
