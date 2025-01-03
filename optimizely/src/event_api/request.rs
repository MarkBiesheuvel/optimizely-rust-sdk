//! Structure for the request payload

// External imports
use error_stack::{Result, ResultExt};

//
use super::EventApiError;
use crate::datafile::Datafile;

// Relative imports of sub modules
use attribute::Attribute;
use decision::Decision;
use event::Event;
pub(crate) use payload::Payload;
use snapshot::Snapshot;
pub(crate) use visitor::Visitor;

mod attribute;
mod decision;
mod event;
mod payload;
mod snapshot;
mod visitor;

// Information about the API endpoint
const ENDPOINT_URL: &str = "https://logx.optimizely.com/v1/events";
const CONTENT_TYPE_KEY: &str = "content-type";
const CONTENT_TYPE_VALUE: &str = "application/json";

/// Representation of an HTTP POST request to the EVENT API
pub struct Request {
    account_id: String,
    payload: Option<Payload>,
}

impl Request {
    /// Create a new empty Request
    pub fn new(datafile: &Datafile) -> Request {
        Request {
            account_id: datafile.account_id().to_owned(),
            payload: Option::None,
        }
    }

    /// Use existing payload or create new one
    fn payload(&mut self) -> &mut Payload {
        self.payload.get_or_insert_with(|| {
            let account_id = self.account_id.clone();
            Payload::new(account_id)
        })
    }

    /// Add a conversion event for a specific visitor to the payload
    pub fn add_conversion_event(&mut self, visitor: Visitor, conversion: &crate::Conversion) {
        self.payload().add_conversion_event(visitor, conversion);
    }

    /// Add a decision event for a specific visitor to the payload
    pub fn add_decision_event(&mut self, visitor: Visitor, decision: &crate::Decision) {
        self.payload().add_decision_event(visitor, decision);
    }

    /// Get the number of visitors in the current payload
    pub fn buffer_size(&self) -> usize {
        match &self.payload {
            Some(payload) => payload.size(),
            None => 0,
        }
    }

    /// Send entire payload
    pub fn send(&mut self) {
        //  Take the payload, so it cannot be send another time
        if let Some(payload) = self.payload.take() {
            // Sending payload
            log::debug!("Sending request to Event API");

            // Convert to payload to string
            let body = match payload.to_string() {
                Ok(body) => body,
                Err(report) => {
                    log::error!("Failed to serialize payload");
                    log::error!("\n{report:?}");
                    return;
                }
            };

            // Make POST request
            let response = Self::post(&body);

            // Send payload to endpoint
            match response {
                Ok(_) => {
                    // TODO: verify status code
                    log::info!("Successfully sent request to Event API");
                }
                Err(report) => {
                    log::error!("Failed to send request to Event API");
                    log::error!("\n{report:?}");
                    return;
                }
            }
        } else {
            log::warn!("Nothing to send");
        }
    }

    fn post(body: &str) -> Result<ureq::Response, EventApiError> {
        ureq::post(ENDPOINT_URL)
            .set(CONTENT_TYPE_KEY, CONTENT_TYPE_VALUE)
            .send_string(body)
            .change_context(EventApiError::FailedRequest)
    }
}

impl Drop for Request {
    fn drop(&mut self) {
        log::debug!("Dropping Request");

        // If the Request is dropped and contained some data, make one last request to the Event API
        if self.payload.is_some() {
            self.send()
        }
    }
}
