//! Event logging to Optimizely Event API

// Relative imports of sub modules
pub use dispatcher::{BatchedEventDispatcher, EventDispatcher, SimpleEventDispatcher};
pub use error::EventApiError;
pub use request::Request;

mod dispatcher;
mod error;
pub mod request;
