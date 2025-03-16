// External imports
use serde::Serialize;

#[derive(Serialize)]
pub struct DecisionEvent {
    campaign_id: String,
    experiment_id: String,
    variation_id: String,
    is_campaign_holdback: bool,
}

impl DecisionEvent {
    pub fn new(campaign_id: String, experiment_id: String, variation_id: String) -> DecisionEvent {
        DecisionEvent {
            campaign_id,
            experiment_id,
            variation_id,
            is_campaign_holdback: false,
        }
    }

    pub(crate) fn campaign_id(&self) -> &str {
        &self.campaign_id
    }
}

impl From<&crate::Decision<'_>> for DecisionEvent {
    fn from(decision: &crate::Decision) -> Self {
        Self::new(decision.campaign_id().into(), decision.experiment_id().into(), decision.variation_id().into())
    }
}
