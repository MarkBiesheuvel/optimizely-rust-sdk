// External imports
use thiserror::Error;

/// This type represents all possible errors that can occur when parsing the datafile
#[derive(Error, Debug, PartialEq)]
pub enum DatafileError {
    #[doc(hidden)]
    #[error("JSON can not be parsed")]
    InvalidJson,
}
