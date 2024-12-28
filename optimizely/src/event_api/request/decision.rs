// External imports
use serde::Serialize;

#[derive(Serialize)]
pub struct Decision {
    campaign_id: String,
    experiment_id: String,
    variation_id: String,
    is_campaign_holdback: bool,
}

impl Decision {
    pub fn new(campaign_id: String, experiment_id: String, variation_id: String) -> Decision {
        Decision {
            campaign_id,
            experiment_id,
            variation_id,
            is_campaign_holdback: false,
        }
    }
}

impl From<&crate::Decision> for Decision {
    fn from(decision: &crate::Decision) -> Self {
        Self::new(decision.campaign_id().into(), decision.experiment_id().into(), decision.variation_id().into())
    }
}
