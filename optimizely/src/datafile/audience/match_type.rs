use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum MatchType {
    AnyValue,
    Exact, // Might need to split up based on type of `value`
    Substring,
    NumberLessThan, // Might need to split up based on type of `value`
    NumberLessThanOrEqual,
    NumberGreaterThan,
    NumberGreaterThanOrEqual,
    SemVerLessThan,
    SemVerLessThanOrEqual,
    SemVerGreaterThan,
    SemVerGreaterThanOrEqual,
}
