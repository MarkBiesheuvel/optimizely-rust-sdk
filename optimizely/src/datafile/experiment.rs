// External imports
use error_stack::Result;
use std::collections::HashMap;
use std::rc::Rc;

// Imports from super
use super::{Context, DatafileError, TrafficAllocation, Variation};

/// Optimizely experiment
#[derive(Debug, Default)]
pub struct Experiment {
    id: String,
    campaign_id: String,
    traffic_allocation: TrafficAllocation,
}

impl Experiment {
    pub(crate) fn new<T: Into<String>>(id: T, campaign_id: T, traffic_allocation: TrafficAllocation) -> Experiment {
        Experiment {
            id: id.into(),
            campaign_id: campaign_id.into(),
            traffic_allocation,
        }
    }

    pub(crate) fn build(context: &mut Context) -> Result<Experiment, DatafileError> {
        // Get fields as string
        let id = context.get("id")?.as_string()?;

        let campaign_id = context.get("layerId")?.as_string()?;

        // TODO: retrieve key
        // TODO: retrieve status and handle different values for status

        // Create map of all variation so they can be looked up within TrafficAllocation
        let mut variations = context
            .get("variations")?
            .as_array()?
            .map(|mut context| Variation::build(&mut context))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|variation| (variation.id().to_owned(), Rc::new(variation)))
            .collect::<HashMap<_, _>>();

        // Build TrafficAllocation struct
        let traffic_allocation = TrafficAllocation::build(context, &mut variations)?;

        Ok(Experiment::new(id, campaign_id, traffic_allocation))
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    #[allow(dead_code)]
    pub fn campaign_id(&self) -> &str {
        &self.campaign_id
    }

    pub fn traffic_allocation(&self) -> &TrafficAllocation {
        &self.traffic_allocation
    }
}
