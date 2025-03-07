// External imports
use serde::Serialize;

// Imports from super
use super::{ConversionEvent, DecisionEvent};

#[derive(Serialize, Default)]
pub struct Snapshot {
    decisions: Vec<DecisionEvent>,
    #[serde(rename = "events")]
    conversions: Vec<ConversionEvent>,
}

impl Snapshot {
    pub fn add_decision_event(&mut self, decision: DecisionEvent) {
        self.decisions.push(decision);
    }

    pub fn add_conversion_event(&mut self, event: ConversionEvent) {
        self.conversions.push(event);
    }
}
