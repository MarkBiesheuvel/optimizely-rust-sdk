// Imports from super
use crate::{client::UserContext, Conversion, Decision};

/// Trait for sending events to Optimizely Event API
///
/// It is possible to make a custom event dispatcher by implementing this trait
/// TODO: add example again
pub trait EventDispatcher: Send + Sync {
    /// Send conversion event to destination
    fn send_conversion_event(&self, user_context: &UserContext, conversion: Conversion);

    /// Send event to destination
    fn send_decision_event(&self, user_context: &UserContext, decision: Decision);
}
