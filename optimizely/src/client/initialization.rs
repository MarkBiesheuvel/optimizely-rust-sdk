use std::time::Duration;
use error_stack::{Result, ResultExt};

// Imports from crate
use crate::client::{Client, ClientError};
use crate::datafile::Datafile;
use crate::decision::DecideOptions;
#[cfg(feature = "online")]
use crate::event_api::EventDispatcher;

/// Intermediate struct that is used to initialize a new [Client].
///
/// See [super] for examples.
pub struct UninitializedClient {
    pub(crate) datafile: Datafile,
    pub(crate) update_interval: Option<Duration>,
    pub(crate) default_decide_options: Option<DecideOptions>,
    #[cfg(feature = "online")]
    pub(crate) event_dispatcher: Option<Box<dyn EventDispatcher>>,
}

impl Client {
    /// Download the datafile from the CDN using an SDK key
    #[cfg(feature = "online")]
    pub fn from_sdk_key(sdk_key: &str) -> Result<UninitializedClient, ClientError> {
        let datafile = Datafile::from_sdk_key(sdk_key).change_context(ClientError::InvalidDatafile)?;
        Client::from_datafile(datafile)
    }

    /// Read the datafile from the local filesystem
    pub fn from_local_datafile(file_path: &str) -> Result<UninitializedClient, ClientError> {
        let datafile = Datafile::from_local_datafile(file_path).change_context(ClientError::InvalidDatafile)?;
        Client::from_datafile(datafile)
    }

    /// Use a string variable as the datafile
    pub fn from_string<S>(content: S) -> Result<UninitializedClient, ClientError>
    where
        S: AsRef<str>,
    {
        let datafile = Datafile::from_string(content).change_context(ClientError::InvalidDatafile)?;
        Client::from_datafile(datafile)
    }

    fn from_datafile(datafile: Datafile) -> Result<UninitializedClient, ClientError> {
        // Return uninitialized client
        Ok(UninitializedClient::new(datafile))
    }
}

impl UninitializedClient {
    pub(super) fn new(datafile: Datafile) -> UninitializedClient {
        UninitializedClient {
            datafile: datafile,
            update_interval: None,
            default_decide_options: None,
            #[cfg(feature = "online")]
            event_dispatcher: None,
        }
    }

    /// Use a custom event dispatcher
    ///
    /// This method accepts a function that can be used to create an EventDispatcher
    ///
    /// If you implement your own EventDispatcher, you could write a method new with the following signature:
    /// `fn new(datafile: &Datafile) -> Self;`
    /// And call this method like so:
    /// `.with_event_dispatcher(BatchedEventDispatcher::new)`
    ///
    /// Or you could call this method with an anonymous function like so:
    /// `.with_event_dispatcher(|_| EventStore::default())`
    #[cfg(feature = "online")]
    pub fn with_event_dispatcher<F, D>(mut self, dispatcher: F) -> UninitializedClient
    where
        F: FnOnce(&Datafile) -> D,
        D: EventDispatcher,
    {
        // Create a new dispatcher of type <D>
        let dispatcher = dispatcher(&self.datafile);

        // Store in a Box<D>, since different EventDispatcher implementations are different types
        self.event_dispatcher = Some(Box::new(dispatcher));

        // Return self, so can chain other functions
        self
    }

    /// Use these decide options for every decide call (if none are specified)
    pub fn with_default_decide_options(mut self, options: DecideOptions) -> UninitializedClient {
        // Store decide options
        self.default_decide_options = Some(options);

        // Return self, so can chain other functions
        self
    }

    /// Automatically fetch the latest datafile in a regular interval
    pub fn with_update_interval(mut self, interval: Duration) -> UninitializedClient {
        // Store interval
        self.update_interval = Some(interval);

        // Return self, so can chain other functions
        self
    }

    /// Initialize the client
    pub fn initialize(self) -> Client {
        Client::from(self)
    }
}
