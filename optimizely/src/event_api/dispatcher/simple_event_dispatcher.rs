// Imports from super
use super::EventDispatcher;

// Imports from crate
use crate::event_api::request::{Payload, Visitor};
use crate::{client::UserContext, Conversion, Decision};

/// Implementation of the EventDispatcher trait that makes an HTTP request for every event
///
/// TODO: add example usage in SDK
pub struct SimpleEventDispatcher {}

impl Default for SimpleEventDispatcher {
    /// Constructor for a new simple event dispatcher
    fn default() -> SimpleEventDispatcher {
        SimpleEventDispatcher {}
    }
}

impl EventDispatcher for SimpleEventDispatcher {
    fn send_conversion_event(&self, user_context: &UserContext, conversion: Conversion) {
        log::debug!("Sending conversion event to Event API");

        // Generate a new payload
        let mut payload = Payload::new(user_context.client().datafile().account_id());

        // Create new request::Visitor
        let visitor = Visitor::from(user_context);

        // Add single conversion
        payload.add_conversion_event(visitor, &conversion);

        // Dispatch single conversion
        payload.send()
    }

    fn send_decision_event(&self, user_context: &UserContext, decision: Decision) {
        log::debug!("Sending decision event to Event API");

        // Generate a new payload
        let mut payload = Payload::new(user_context.client().datafile().account_id());

        // Create new request::Visitor
        let visitor = Visitor::from(user_context);

        // Add single decision
        payload.add_decision_event(visitor, &decision);

        // Dispatch single decision
        payload.send()
    }
}
