use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum MatchType {
    Exists,
    Exact,
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
