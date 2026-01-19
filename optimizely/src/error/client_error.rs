// External imports
use thiserror::Error;

/// Representation of client initialization errors.
#[derive(Error, Debug, PartialEq)]
pub enum ClientError {
    /// Invalid Datafile
    #[error("Invalid Datafile")]
    InvalidDatafile,
}
