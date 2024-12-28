//! Event logging to Optimizely Event API

// External imports
use error_stack::{Result, ResultExt};

// Relative imports of sub modules
pub use dispatcher::{BatchedEventDispatcher, EventDispatcher, SimpleEventDispatcher};
pub use error::EventApiError;
use request::Payload;

mod dispatcher;
mod error;
pub mod request;

// Information about the API endpoint
const ENDPOINT_URL: &str = "https://logx.optimizely.com/v1/events";
const CONTENT_TYPE_KEY: &str = "content-type";
const CONTENT_TYPE_VALUE: &str = "application/json";

/// Serialize the payload to JSON and send to Event API
fn send(payload: &Payload) -> Result<(), EventApiError> {
    // Convert to JSON document and dump as String
    let body = serde_json::to_string(payload).change_context(EventApiError::FailedSerialize)?;

    // Make POST request
    ureq::post(ENDPOINT_URL)
        .set(CONTENT_TYPE_KEY, CONTENT_TYPE_VALUE)
        .send_string(&body)
        .change_context(EventApiError::FailedRequest)?;

    Ok(())
}

impl Payload<'_> {
    /// Send entire payload
    pub fn send(&self) {
        // Sending payload
        log::debug!("Sending request to Event API");

        // Send payload to endpoint
        match send(self) {
            Ok(_) => {
                log::info!("Successfully sent request to Event API");
            }
            Err(report) => {
                log::error!("Failed to send request to Event API");
                log::error!("\n{report:?}");
            }
        }
    }
}
