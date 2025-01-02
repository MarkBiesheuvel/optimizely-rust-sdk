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
