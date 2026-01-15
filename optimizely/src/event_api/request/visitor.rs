use serde::Serialize;

// Imports from crate
use crate::{client::UserContext, Conversion, Decision, UserAttribute};

// Imports from super
use super::Snapshot;

#[derive(Serialize)]
pub struct Visitor {
    visitor_id: String,
    attributes: Vec<UserAttribute>,
    snapshots: [Snapshot; 1],
}

impl Visitor {
    pub fn new(visitor_id: String, attributes: Vec<UserAttribute>) -> Visitor {
        Visitor {
            visitor_id,
            attributes,
            snapshots: [Snapshot::default()],
        }
    }

    pub fn add_decision_event(&mut self, decision: Decision) {
        self.snapshots[0].add_decision_event(decision);
    }

    pub fn add_conversion_event(&mut self, event: Conversion) {
        self.snapshots[0].add_conversion_event(event);
    }
}

impl From<&UserContext<'_>> for Visitor {
    fn from(user: &UserContext) -> Self {
        let user_id = user.user_id().into();
        let attributes = user
            .user_attributes()
            .into_iter()
            .map(|attribute| attribute.clone())
            .collect::<Vec<_>>();

        Visitor::new(user_id, attributes)
    }
}
