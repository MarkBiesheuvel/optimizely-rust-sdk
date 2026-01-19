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
    SemVer(SemVerOperator),
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum SemVerOperator {
    Equal,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}
