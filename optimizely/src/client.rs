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

#[cfg(feature = "online")]
use crate::event_api::EventDispatcher;
use crate::{datafile::Datafile, event_api::SimpleEventDispatcher};
use std::sync::{RwLock, RwLockReadGuard};

// Relative imports of sub modules
pub use error::ClientError;
pub use initialization::UninitializedClient;
pub use user_attribute::{UserAttribute, UserAttributeMap};
pub use user_context::UserContext;

mod error;
mod initialization;
mod user_attribute;
mod user_context;

/// SDK client to interact with feature flags.
///
/// See [super] for examples.
pub struct Client {
    datafile: RwLock<Datafile>,
    #[cfg(feature = "online")]
    event_dispatcher: Box<dyn EventDispatcher>,
}

type DatafileReadLock<'a> = RwLockReadGuard<'a, Datafile>;

impl From<UninitializedClient> for Client {
    fn from(options: UninitializedClient) -> Self {
        let datafile = options.datafile;

        // Select default for any options that were not specified
        #[cfg(feature = "online")]
        let event_dispatcher = options
            .event_dispatcher
            .unwrap_or_else(|| Box::new(SimpleEventDispatcher::new(&datafile)));

        Client {
            datafile: RwLock::new(datafile),
            #[cfg(feature = "online")]
            event_dispatcher,
        }
    }
}

impl Client {
    /// Create a new user context for a given user id
    pub fn create_user_context<'a>(&'a self, user_id: &'a str) -> UserContext<'a> {
        UserContext::new(self, user_id)
    }

    /// Get the datafile within the client
    pub fn datafile(&self) -> DatafileReadLock<'_> {
        // Obtain read lock
        let lock_result = self.datafile.read();

        // The lock should not be poisoned, since the writing thread should not panic
        lock_result.expect("The read/write lock on datafile should not be poisoned.")
    }

    /// Get the event dispatcher within the client
    #[cfg(feature = "online")]
    pub fn event_dispatcher(&self) -> &dyn EventDispatcher {
        &*self.event_dispatcher
    }
}
