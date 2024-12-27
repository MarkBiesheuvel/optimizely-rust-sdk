//! Event logging to Optimizely Event API

// Relative imports of sub modules
pub use batched_event_dispatcher::BatchedEventDispatcher;
pub use client::EventApiClient;
pub use error::EventApiError;
pub use simple_event_dispatcher::SimpleEventDispatcher;
pub use trait_event_dispatcher::EventDispatcher;

mod batched_event_dispatcher;
mod client;
mod error;
pub mod request;
mod simple_event_dispatcher;
mod trait_event_dispatcher;
