//! Structure for the request payload

// Relative imports of sub modules
use attribute::Attribute;
use decision::Decision;
use event::Event;
pub(crate) use payload::Payload;
use snapshot::Snapshot;
pub(crate) use visitor::Visitor;

mod attribute;
mod decision;
mod event;
mod payload;
mod snapshot;
mod visitor;
