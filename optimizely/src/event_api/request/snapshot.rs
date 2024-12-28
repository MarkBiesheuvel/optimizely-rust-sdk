// External imports
use serde::Serialize;

// Imports from super
use super::{Decision, Event};

#[derive(Serialize, Default)]
pub struct Snapshot {
    decisions: Vec<Decision>,
    events: Vec<Event>,
}

impl Snapshot {
    pub fn add_decision(&mut self, decision: &crate::Decision) {
        self.decisions.push(Decision::from(decision));
    }

    pub fn add_event(&mut self, conversion: &crate::Conversion) {
        self.events.push(Event::from(conversion));
    }
}
