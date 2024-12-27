#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![warn(missing_docs)]

// Reimport/export of structs to make them available at top-level
pub use client::Client;
#[cfg(feature = "online")]
pub use conversion::Conversion;
pub use decision::Decision;

// Regular modules
pub mod client;
#[cfg(feature = "online")]
pub mod conversion;
pub mod datafile;
pub mod decision;

#[cfg(feature = "online")]
pub mod event_api;
