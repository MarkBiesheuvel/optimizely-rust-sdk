// External imports
use error_stack::{Result, ResultExt};

// Imports from super
use super::{request::Payload, EventApiError};

// Information about the API endpoint
const ENDPOINT_URL: &str = "https://logx.optimizely.com/v1/events";
const CONTENT_TYPE_KEY: &str = "content-type";
const CONTENT_TYPE_VALUE: &str = "application/json";

/// HTTP client for the Event API
pub struct EventApiClient {}

impl EventApiClient {
    /// Serialize the payload to JSON and send to Event API
    pub fn send(payload: &Payload) -> Result<(), EventApiError> {
        // Convert to JSON document and dump as String
        let body = serde_json::to_string(payload).change_context(EventApiError::FailedSerialize)?;

        // Make POST request
        ureq::post(ENDPOINT_URL)
            .set(CONTENT_TYPE_KEY, CONTENT_TYPE_VALUE)
            .send_string(&body)
            .change_context(EventApiError::FailedRequest)?;

        Ok(())
    }
}
