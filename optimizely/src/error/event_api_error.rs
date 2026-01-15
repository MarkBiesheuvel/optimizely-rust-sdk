// External imports
use thiserror::Error;

/// This type represents all possible errors that can occur when communicating with Event API
#[derive(Error, Debug, PartialEq)]
pub enum EventApiError {
    #[doc(hidden)]
    #[error("Failed to send request to Event API")]
    FailedRequest,
    #[doc(hidden)]
    #[error("Failed to serialize payload to JSON")]
    FailedSerialize,
}
