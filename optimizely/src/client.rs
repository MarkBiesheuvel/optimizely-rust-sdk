//! The interface that enables you to interact with feature flags.
//!
//! # Initialization
//!
//! An SDK client has one required property: a [Datafile] and a few additional optional properties.
//!
//! Therefore, you first call one of the `Client::from_*` functions, which returns an [UninitializedClient].
//! Then, you can than add any number of optional properties using the `UninitializedClient::with_*` methods.
//! Finally, you complete the SDK client by calling `UninitializedClient::initialize`.
//!
//! # Examples
//!
//! Creating a simple SDK client.
//! ```
//! use optimizely::Client;
//! # const SDK_KEY: &str = "KVpGWnzPGKvvQ8yeEWmJZ";
//!
//! let client = Client::from_sdk_key(SDK_KEY)?
//!     .initialize();
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! Creating an SDK client using a local file
//! ```
//! use optimizely::Client;
//!
//! let client = Client::from_local_datafile("../datafiles/sandbox.json")?
//!     .initialize();
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! Creating an SDK client with the batched event dispatcher
//! ```
//! use optimizely::{event_api::BatchedEventDispatcher, Client};
//! # const SDK_KEY: &str = "KVpGWnzPGKvvQ8yeEWmJZ";
//!
//! // Initiate client using SDK key and batched event dispatcher
//! let client = Client::from_sdk_key(SDK_KEY)?
//!     .with_event_dispatcher(BatchedEventDispatcher::new)
//!     .initialize();
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

// Imports from crate
use crate::datafile::Datafile;
#[cfg(feature = "online")]
use crate::event_api::EventDispatcher;

// Relative imports of sub modules
pub use error::ClientError;
pub use initialization::UninitializedClient;
pub use user_attribute::UserAttribute;
pub use user_context::UserContext;

mod error;
mod initialization;
mod user_attribute;
mod user_context;

/// SDK client to interact with feature flags.
///
/// See [super] for examples.
pub struct Client {
    datafile: Datafile,
    #[cfg(feature = "online")]
    event_dispatcher: Box<dyn EventDispatcher>,
}

impl Client {
    /// Create a new user context for a given user id
    pub fn create_user_context<'a>(&'a self, user_id: &'a str) -> UserContext<'a> {
        UserContext::new(self, user_id)
    }

    // /// Create a new user context for a given user id
    // pub fn create_user_context_with_attributes<'a>(
    //     &'a self, user_id: &'a str, attributes: UserAttributes,
    // ) -> UserContext<'a> {
    //     UserContext::new(self, user_id, attributes)
    // }

    /// Get the datafile within the client
    pub fn datafile(&self) -> &Datafile {
        &self.datafile
    }

    /// Get the event dispatcher within the client
    #[cfg(feature = "online")]
    pub fn event_dispatcher(&self) -> &dyn EventDispatcher {
        &*self.event_dispatcher
    }
}
