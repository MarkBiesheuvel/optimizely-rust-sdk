// External imports
use serde::Serialize;

use crate::{Conversion as CrateConversion, Decision as CrateDecision};

// Imports from super
use super::{Decision as PayloadDecision, Event as PayloadEvent};

#[derive(Serialize, Default)]
pub struct Snapshot {
    decisions: Vec<PayloadDecision>,
    events: Vec<PayloadEvent>,
}

impl Snapshot {
    pub fn new() -> Snapshot {
        Snapshot::default()
    }

    pub fn add_decision(&mut self, decision: &CrateDecision) {
        // TODO: impl From trait
        let decision = PayloadDecision::new(
            decision.campaign_id().into(),
            decision.experiment_id().into(),
            decision.variation_id().into(),
        );
        self.decisions.push(decision);
    }

    pub fn add_event(&mut self, conversion: &CrateConversion) {
        // TODO: impl From trait
        let event = PayloadEvent::new(
            conversion.event_id().into(),
            conversion.event_key().into(),
            conversion.properties().clone(),
            conversion.tags().clone(),
        );
        self.events.push(event);
    }
}
