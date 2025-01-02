use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub enum NumericOperator {
    Equal,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum StringOperator {
    Equal,
    Contains,
    SemVerEqual,
    SemVerLessThan,
    SemVerLessThanOrEqual,
    SemVerGreaterThan,
    SemVerGreaterThanOrEqual,
}
