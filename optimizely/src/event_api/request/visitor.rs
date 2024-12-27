// External imports
use serde::Serialize;

use crate::{Conversion, Decision};

// Imports from super
use super::Snapshot;

#[derive(Serialize)]
pub struct Visitor {
    visitor_id: String,
    // TODO: add field `attributes`
    snapshots: [Snapshot; 1],
}

impl Visitor {
    pub fn new<T: Into<String>>(visitor_id: T) -> Visitor {
        Visitor {
            visitor_id: visitor_id.into(),
            snapshots: [Snapshot::new()],
        }
    }

    pub fn add_decision(&mut self, decision: &Decision) {
        self.snapshots[0].add_decision(decision);
    }

    pub fn add_event(&mut self, conversion: &Conversion) {
        self.snapshots[0].add_event(conversion);
    }
}
