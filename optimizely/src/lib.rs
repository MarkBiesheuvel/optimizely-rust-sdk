#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![warn(missing_docs)]

pub use client::Client;
#[cfg(feature = "online")]
pub use conversion::Conversion;
pub use datafile::AttributeValue;
pub use decision::Decision;

pub mod client;
#[cfg(feature = "online")]
mod conversion;
pub mod datafile;
pub mod decision;
#[cfg(feature = "online")]
pub mod event_api;
