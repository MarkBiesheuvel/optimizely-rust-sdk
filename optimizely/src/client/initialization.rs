// External imports
use error_stack::{Result, ResultExt};
use std::fs::File;
use std::io::Read;

// Imports from crate
use crate::client::{Client, ClientError};
use crate::datafile::Datafile;

#[cfg(feature = "online")]
use crate::event_api::{EventDispatcher, SimpleEventDispatcher};

/// An intermediate struct that is returned when building a new Client
///
/// ```
/// use optimizely::Client;
/// use optimizely::event_api::BatchedEventDispatcher;
///
/// // Initialize Optimizely client using local datafile and custom event dispatcher
/// let file_path = "../datafiles/sandbox.json";
/// let optimizely_client = Client::from_local_datafile(file_path)?
///     .with_event_dispatcher(BatchedEventDispatcher::new)
///     .initialize();
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct UninitializedClient {
    datafile: Datafile,
    _default_decide_options: Option<()>,
    _user_profile_service: Option<()>,
    #[cfg(feature = "online")]
    event_dispatcher: Option<Box<dyn EventDispatcher>>,
}

impl Client {
    /// Download the datafile from the CDN using an SDK key
    #[cfg(feature = "online")]
    pub fn from_sdk_key(sdk_key: &str) -> Result<UninitializedClient, ClientError> {
        // Construct URL
        let url = format!("https://cdn.optimizely.com/datafiles/{}.json", sdk_key);

        // Make GET request
        // TODO: implement polling mechanism
        let response = ureq::get(&url)
            .call()
            .change_context(ClientError::FailedRequest)?;

        // Get response body
        let content = response
            .into_string()
            .change_context(ClientError::FailedResponse)?;

        // Use response to build Client
        Client::from_string(&content)
    }

    /// Read the datafile from the local filesystem
    pub fn from_local_datafile(file_path: &str) -> Result<UninitializedClient, ClientError> {
        // Read content from local path
        let mut content = String::new();

        // Open file
        let mut file = File::open(file_path).change_context(ClientError::FailedFileOpen)?;

        // Read file content into String
        file.read_to_string(&mut content)
            .change_context(ClientError::FailedFileRead)?;

        // Use file content to build Client
        Client::from_string(&content)
    }

    /// Use a string variable as the datafile
    pub fn from_string(content: &str) -> Result<UninitializedClient, ClientError> {
        // Create datafile from a string
        let datafile = Datafile::build(content).change_context(ClientError::InvalidDatafile)?;

        // Return uninitialized client
        Ok(UninitializedClient::new(datafile))
    }
}

impl UninitializedClient {
    pub(super) fn new(datafile: Datafile) -> UninitializedClient {
        UninitializedClient {
            datafile,
            _default_decide_options: None,
            _user_profile_service: None,
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

    // TODO: implement with_default_decide_options and with_user_profile_service

    /// Initialize the client
    pub fn initialize(self) -> Client {
        let datafile = self.datafile;

        // Select default for any options that were not specified
        let event_dispatcher = match self.event_dispatcher {
            Some(event_dispatcher) => event_dispatcher,
            None => Box::new(SimpleEventDispatcher::new(&datafile)),
        };

        Client {
            datafile: datafile,
            #[cfg(feature = "online")]
            event_dispatcher,
        }
    }
}
