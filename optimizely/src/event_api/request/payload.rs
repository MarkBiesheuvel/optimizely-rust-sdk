// External imports
use serde::Serialize;
use std::collections::HashMap;

// Imports from super
use super::Visitor;
use crate::{event_api::EventApiClient, Conversion, Decision};

// Information regarding the SDK client
const CLIENT_NAME: &str = "rust-sdk";
const CLIENT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Event key for activating an experiment
const ACTIVATE_EVENT_KEY: &str = "campaign_activated";

#[derive(Serialize)]
/// HTTP request payload to send to Event API
pub struct Payload<'a> {
    account_id: String,
    visitors: Vec<Visitor>,
    enrich_decisions: bool,
    anonymize_ip: bool,
    client_name: &'a str,
    client_version: &'a str,
}

impl Payload<'_> {
    /// Construct an empty payload for a given account
    pub fn new<T: Into<String>>(account_id: T) -> Payload<'static> {
        Payload {
            account_id: account_id.into(),
            visitors: Vec::<Visitor>::new(),
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
    pub fn add_conversion_event<T: Into<String>>(&mut self, user_id: T, conversion: &Conversion) {
        log::debug!("Adding conversion event to payload");
        // TODO: look up visitor ID in existing list

        // Create new request::Visitor
        let mut visitor = Visitor::new(user_id);

        // Add custom event
        visitor.add_event(conversion);

        // Add to the list
        self.visitors.push(visitor);
    }

    /// Add a decision event for a specific visitor to the payload
    pub fn add_decision_event<T: Into<String>>(&mut self, user_id: T, decision: &Decision) {
        log::debug!("Adding decision event to payload");
        // TODO: look up visitor ID in existing list

        // Create new request::Visitor
        let mut visitor = Visitor::new(user_id);

        // Use campaign_id as entity_id
        let entity_id = decision.campaign_id();

        // Add decision to visitor
        visitor.add_decision(decision);

        // Campaign activated event does not have tags or properties
        let properties = HashMap::default();
        let tags = HashMap::default();

        // Add campaign_activated event
        let conversion = Conversion::new(ACTIVATE_EVENT_KEY, entity_id, properties, tags);
        visitor.add_event(&conversion);

        // Add to the list
        self.visitors.push(visitor);
    }

    /// Send entire payload
    pub fn send(&self) {
        // Sending payload
        log::debug!("Sending request to Event API");

        // Send payload to endpoint
        match EventApiClient::send(self) {
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

impl Drop for Payload<'_> {
    fn drop(&mut self) {
        log::debug!("Dropping Payload");

        // If the Payload is dropped, make one last request to the Event API
        self.send()
    }
}
