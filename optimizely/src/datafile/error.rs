// External imports
use thiserror::Error;

/// This type represents all possible errors that can occur when parsing the datafile
#[derive(Error, Debug, PartialEq)]
pub enum DatafileError {
    /// Failed to parse JSON
    #[error("JSON can not be parsed")]
    InvalidJson,
    /// Failed to make request to cdn.optimizely.com
    #[error("Failed to make request to cdn.optimizely.com")]
    FailedRequest,
    /// Failed to decode response from cdn.optimizely.com
    #[error("Failed to decode response from cdn.optimizely.com")]
    FailedResponse,
    /// Failed to open local datafile
    #[error("Failed to open local datafile")]
    FailedFileOpen,
    /// Failed to read from local datafile
    #[error("Failed to read from local datafile")]
    FailedFileRead,
}

impl serde::de::Error for DatafileError {
    fn custom<T: std::fmt::Display>(_msg: T) -> Self {
        DatafileError::InvalidJson
    }
}
