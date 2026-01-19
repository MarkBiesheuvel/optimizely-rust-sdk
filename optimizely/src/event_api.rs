//! Event logging to Optimizely Event API

// Relative imports of sub modules
pub use dispatcher::*;

mod dispatcher;
pub(crate) mod request;
