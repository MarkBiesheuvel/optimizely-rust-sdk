#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![warn(missing_docs)]

// Re-export the Client
pub use client::Client;

// Re-export all types
pub use types::*;

// Standard modules
pub mod client;
pub mod datafile;
mod types;

// Optional module
#[cfg(feature = "online")]
pub mod event_api;
