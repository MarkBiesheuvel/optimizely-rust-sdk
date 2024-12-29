use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(tag = "match")]
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
    #[serde(rename = "semver_lt")]
    SemVerLessThan,
    #[serde(rename = "semver_le")]
    SemVerLessThanOrEqual,
    #[serde(rename = "semver_gt")]
    SemVerGreaterThan,
    #[serde(rename = "semver_ge")]
    SemVerGreaterThanOrEqual,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_parse_test() {
        let json = r#"{"match":"semver_ge","name":"app_version","type":"custom_attribute","value":"0.4.0"}"#;
        let foo: MatchType = serde_json::from_str(json).unwrap();

        assert_eq!(foo, MatchType::SemVerGreaterThanOrEqual);
    }
}
