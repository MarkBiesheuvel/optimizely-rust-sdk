// External imports
use thiserror::Error;

/// Representation of client initialization errors.
#[derive(Error, Debug, PartialEq)]
pub enum ClientError {
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
    /// Invalid Datafile
    #[error("Invalid Datafile")]
    InvalidDatafile,
}
