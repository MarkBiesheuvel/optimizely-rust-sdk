//! Event logging to Optimizely Event API

// Relative imports of sub modules
pub use dispatcher::{BatchedEventDispatcher, EventDispatcher, SimpleEventDispatcher};
pub(crate) use error::EventApiError;

mod dispatcher;
mod error;
pub(crate) mod request;
