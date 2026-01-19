//! Collection of all `Error` types

pub use client_error::ClientError;
pub use datafile_error::DatafileError;
pub use event_api_error::EventApiError;

mod client_error;
mod datafile_error;
mod event_api_error;
