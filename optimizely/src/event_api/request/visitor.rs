// External imports
use serde::Serialize;

// Imports from crate
use crate::{client::UserContext, Conversion, Decision};

// Imports from super
use super::{Attribute, Snapshot};

#[derive(Serialize)]
pub struct Visitor {
    visitor_id: String,
    attributes: Vec<Attribute>,
    snapshots: [Snapshot; 1],
}

impl Visitor {
    pub fn new(visitor_id: String, attributes: Vec<Attribute>) -> Visitor {
        Visitor {
            visitor_id,
            attributes,
            snapshots: [Snapshot::default()],
        }
    }

    pub fn add_decision(&mut self, decision: &Decision) {
        self.snapshots[0].add_decision(decision);
    }

    pub fn add_event(&mut self, conversion: &Conversion) {
        self.snapshots[0].add_event(conversion);
    }
}

impl From<&UserContext<'_>> for Visitor {
    fn from(user: &UserContext) -> Self {
        let user_id = user.user_id().into();
        let attributes = user
            .user_attributes()
            .into_iter()
            .map(|user_attribute| Attribute::from(user_attribute))
            .collect();

        Visitor::new(user_id, attributes)
    }
}
