// External imports
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum DatafileError {
    #[error("Missing field in datafile: {0:?}")]
    MissingField(String),
    #[error("Revision is not parsable as integer")]
    InvalidRevision,
    #[error("Rollout ID does not exist: {0:?}")]
    InvalidRolloutId(String),
    #[error("Experiment ID does not exist: {0:?}")]
    InvalidExperimentId(String),
    #[error("Variation ID does not exist: {0:?}")]
    InvalidVariationId(String),
}
