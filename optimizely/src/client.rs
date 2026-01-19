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

// External imports
use std::sync::{Arc, RwLock, RwLockReadGuard};
use std::thread::{self, sleep};

// Imports from crate
use crate::{datafile::Datafile, DecideOptions};

// Optional import
#[cfg(feature = "online")]
use crate::event_api::{EventDispatcher, SimpleEventDispatcher};

// Relative imports of sub modules
pub use initialization::UninitializedClient;
pub use user_context::UserContext;

mod initialization;
mod user_context;

/// SDK client to interact with feature flags.
///
/// See [super] for examples.
pub struct Client {
    datafile_lock: Arc<RwLock<Datafile>>,
    default_decide_options: DecideOptions,
    #[cfg(feature = "online")]
    event_dispatcher: Box<dyn EventDispatcher>,
}

type DatafileReadGuard<'a> = RwLockReadGuard<'a, Datafile>;

impl From<UninitializedClient> for Client {
    fn from(options: UninitializedClient) -> Self {
        // Select default for any options that were not specified
        #[cfg(feature = "online")]
        let event_dispatcher = options
            .event_dispatcher
            .unwrap_or_else(|| Box::new(SimpleEventDispatcher::new(&options.datafile)));

        let default_decide_options = options.default_decide_options.unwrap_or_default();

        // Clone SDK key so it can be moved to the polling thread
        let sdk_key = options.datafile.sdk_key().to_owned();
        let mut current_revision = options.datafile.revision();

        // Store the datafile in a reference counted read/write lock
        let datafile_lock = Arc::new(RwLock::new(options.datafile));

        // Clone the reference
        let datafile_lock_clone = datafile_lock.clone();

        // TODO: make auto update only possible when the online feature is enabled
        // Spawn a thread to update the datafile in the background if update interval is set
        if let Some(interval) = options.update_interval {
            thread::spawn(move || {
                log::debug!("Starting thread for datafile polling");

                loop {
                    log::debug!("Fetching latest datafile");

                    // Request new datafile
                    if let Ok(datafile) = Datafile::from_sdk_key(&sdk_key) {
                        let latest_revision = datafile.revision();

                        // Only acquire write lock if revision changed
                        if current_revision < latest_revision {
                            log::info!("Updating datafile from {current_revision} to {latest_revision}");
                            if let Ok(mut lock_guard) = datafile_lock_clone.write() {
                                *lock_guard = datafile;
                                current_revision = latest_revision;
                            } else {
                                log::error!("Failed to acquire write lock on datafile")
                            }
                        }
                    }

                    sleep(interval);
                }
            });
        }

        Client {
            datafile_lock,
            default_decide_options,
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
    pub fn datafile(&self) -> DatafileReadGuard<'_> {
        // Obtain read lock
        let lock_result = self.datafile_lock.read();

        // The lock should not be poisoned, since the writing thread should not panic
        lock_result.expect("The read/write lock on datafile should not be poisoned.")
    }

    /// Get the default DecideOptions
    pub fn default_decide_options(&self) -> &DecideOptions {
        &self.default_decide_options
    }

    /// Get the event dispatcher within the client
    #[cfg(feature = "online")]
    pub fn event_dispatcher(&self) -> &dyn EventDispatcher {
        &*self.event_dispatcher
    }
}
