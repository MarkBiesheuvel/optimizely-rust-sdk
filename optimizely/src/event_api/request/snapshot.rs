// External imports
use serde::Serialize;

// Imports from crate
use crate::{Conversion, Decision};

#[derive(Serialize, Default)]
pub struct Snapshot {
    decisions: Vec<Decision>,
    #[serde(rename = "events")]
    conversions: Vec<Conversion>,
}

impl Snapshot {
    pub fn add_decision_event(&mut self, decision: Decision) {
        self.decisions.push(decision);
    }

    pub fn add_conversion_event(&mut self, event: Conversion) {
        self.conversions.push(event);
    }
}
