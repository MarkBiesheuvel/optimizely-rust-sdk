use std::sync::RwLock;

// Imports from super
use super::EventDispatcher;

// Imports from crate
use crate::event_api::request::{Request, Visitor};
use crate::{client::UserContext, Conversion, Decision};

/// Implementation of the EventDispatcher trait that makes an HTTP request for every event
///
/// TODO: add example usage in SDK
pub struct SimpleEventDispatcher {
    request: RwLock<Request>,
}

impl SimpleEventDispatcher {
    /// Constructor for a new simple event dispatcher
    pub fn new(datafile: &crate::datafile::Datafile) -> SimpleEventDispatcher {
        // Generate a new payload
        let request = RwLock::new(Request::new(datafile));

        SimpleEventDispatcher { request }
    }
}

impl EventDispatcher for SimpleEventDispatcher {
    fn send_conversion_event(&self, user_context: &UserContext, conversion: Conversion) {
        log::debug!("Sending conversion event to Event API");

        // Get mutable reference to request
        let mut request = match self.request.try_write() {
            Ok(request) => request,
            Err(_) => {
                log::error!("Unable to dispatch concurrently");
                return;
            }
        };

        // Create new request::Visitor
        let visitor = Visitor::from(user_context);

        // Add single conversion
        request.add_conversion_event(visitor, conversion);

        // Dispatch single conversion
        request.send()
    }

    fn send_decision_event(&self, user_context: &UserContext, decision: Decision) {
        log::debug!("Sending decision event to Event API");

        // Get mutable reference to request
        let mut request = match self.request.try_write() {
            Ok(request) => request,
            Err(_) => {
                log::error!("Unable to dispatch concurrently");
                return;
            }
        };

        // Create new request::Visitor
        let visitor = Visitor::from(user_context);

        // Add single decision
        request.add_decision_event(visitor, decision);

        // Dispatch single decision
        request.send()
    }
}
