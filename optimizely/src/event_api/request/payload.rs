// External imports
use error_stack::{Result, ResultExt};
use serde::Serialize;
use std::collections::HashMap;

use crate::event_api::EventApiError;

// Imports from super
use super::{ConversionEvent, DecisionEvent, Visitor};

// Information regarding the SDK client
const CLIENT_NAME: &str = "rust-sdk";
const CLIENT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Event key for activating an experiment
const ACTIVATE_EVENT_KEY: &str = "campaign_activated";

#[derive(Serialize)]
/// HTTP request payload to send to Event API
pub struct Payload {
    account_id: String,
    visitors: Vec<Visitor>,
    enrich_decisions: bool,
    anonymize_ip: bool,
    client_name: &'static str,
    client_version: &'static str,
}

impl Payload {
    /// Construct an empty payload for a given account
    pub fn new(account_id: String) -> Payload {
        Payload {
            account_id: account_id,
            visitors: Vec::new(),
            enrich_decisions: true,
            anonymize_ip: true,
            client_name: CLIENT_NAME,
            client_version: CLIENT_VERSION,
        }
    }

    /// Return the number of visitors in the payload
    pub fn size(&self) -> usize {
        self.visitors.len()
    }

    /// Add a conversion event for a specific visitor to the payload
    pub fn add_conversion_event(&mut self, mut visitor: Visitor, event: ConversionEvent) {
        log::debug!("Adding conversion event to payload");

        // Add custom event
        visitor.add_conversion_event(event);

        // Add to the list
        self.visitors.push(visitor);
    }

    /// Add a decision event for a specific visitor to the payload
    pub fn add_decision_event(&mut self, mut visitor: Visitor, decision: DecisionEvent) {
        log::debug!("Adding decision event to payload");

        // Copy campaign_id as entity_id
        let entity_id = decision.campaign_id().to_owned();

        // Add decision to visitor
        visitor.add_decision_event(decision);

        // Campaign activated event does not have tags or properties
        let properties = HashMap::default();
        let tags = HashMap::default();

        // Add campaign_activated event
        let event = ConversionEvent::new(entity_id, ACTIVATE_EVENT_KEY.into(), properties, tags);
        visitor.add_conversion_event(event);

        // Add to the list
        self.visitors.push(visitor);
    }

    /// Convert the Payload struct to a JSON encoded text
    pub fn to_string(&self) -> Result<String, EventApiError> {
        // Convert to JSON document and dump as String
        serde_json::to_string(self).change_context(EventApiError::FailedSerialize)
    }
}
