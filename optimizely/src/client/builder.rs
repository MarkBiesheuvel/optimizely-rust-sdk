// External imports
use error_stack::{IntoReport, Result, ResultExt};
use std::fs::File;
use std::io::Read;
use std::marker::PhantomData;

// Imports from crate
use crate::client::{Client, ClientError};
use crate::datafile::{Datafile, Json};

#[cfg(feature = "online")]
use crate::event_api::{EventDispatcher, SimpleEventDispatcher};

/// State of ClientBuilder indicating that the datafile is still empty.
///
/// The `.build` is not implemented on `ClientBuilder<Empty>`.
/// Inspired by: <https://youtu.be/_ccDqRTx-JU>
#[derive(Default)]
pub struct Empty;
/// State of ClientBuilder indicating that the datafile is present/ready.
///
/// The `.build` is only implemented on `ClientBuilder<Ready>`.
/// Inspired by: <https://youtu.be/_ccDqRTx-JU>
pub struct Ready;

/// Factory/builder pattern for the SDK client
///
/// ```
/// use optimizely::ClientBuilder;
/// use optimizely::event_api::BatchedEventDispatcher;
///
/// // Initialize Optimizely client using local datafile and custom event dispatcher
/// let file_path = "../datafiles/sandbox.json";
/// let event_dispatcher = BatchedEventDispatcher::default();
/// let optimizely_client = ClientBuilder::new()
///     .with_local_datafile(file_path)?
///     .with_event_dispatcher(event_dispatcher)
///     .build();
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Default)]
pub struct ClientBuilder<State = Empty> {
    datafile: Option<Datafile>,
    state: PhantomData<State>,
    #[cfg(feature = "online")]
    event_dispatcher: Option<Box<dyn EventDispatcher>>,
}

impl ClientBuilder {
    /// Constructor for a new client factory/builder
    pub fn new() -> ClientBuilder<Empty> {
        ClientBuilder::default()
    }
}

impl ClientBuilder<Empty> {
    /// Download the datafile from the CDN using an SDK key
    #[cfg(feature = "online")]
    pub fn with_sdk_key(self, sdk_key: &str) -> Result<ClientBuilder<Ready>, ClientError> {
        // Construct URL
        let url = format!("https://cdn.optimizely.com/datafiles/{}.json", sdk_key);

        // Make GET request
        // TODO: implement polling mechanism
        let response = ureq::get(&url)
            .call()
            .into_report()
            .change_context(ClientError::FailedRequest)?;

        // Get response body
        let content = response
            .into_string()
            .into_report()
            .change_context(ClientError::FailedResponse)?;

        // Use response to build Client
        self.with_datafile_as_string(&content)
    }

    /// Read the datafile from the local filesystem
    pub fn with_local_datafile(self, file_path: &str) -> Result<ClientBuilder<Ready>, ClientError> {
        // Read content from local path
        let mut content = String::new();

        // Open file
        let mut file = File::open(file_path)
            .into_report()
            .change_context(ClientError::FailedFileOpen)?;

        // Read file content into String
        file.read_to_string(&mut content)
            .into_report()
            .change_context(ClientError::FailedFileRead)?;

        // Use file content to build Client
        self.with_datafile_as_string(&content)
    }

    /// Use a string variable as the datafile
    pub fn with_datafile_as_string(self, content: &str) -> Result<ClientBuilder<Ready>, ClientError> {
        // Parse content as JSON
        let mut json = Json::build(content).change_context(ClientError::InvalidDatafile)?;

        // Create datafile from JSON value
        let datafile = Datafile::build(&mut json).change_context(ClientError::InvalidDatafile)?;

        Ok(self.with_native_datafile(datafile))
    }

    /// Use a native Datafile struct
    pub fn with_native_datafile(self, datafile: Datafile) -> ClientBuilder<Ready> {
        ClientBuilder {
            datafile: Some(datafile),
            state: PhantomData,
            #[cfg(feature = "online")]
            event_dispatcher: self.event_dispatcher,
        }
    }
}

impl<State> ClientBuilder<State> {
    /// Use a custom event dispatcher
    #[cfg(feature = "online")]
    pub fn with_event_dispatcher(mut self, event_dispatcher: impl EventDispatcher + 'static) -> ClientBuilder<State> {
        self.event_dispatcher = Some(Box::new(event_dispatcher));
        self
    }
}

impl ClientBuilder<Ready> {
    /// Build the client
    pub fn build(self) -> Client {
        // Retrieve content from build options
        let datafile = self
            .datafile
            .expect("Datafile is garantueed to be Some(_) since the state is ready.");

        #[cfg(feature = "online")]
        let event_dispatcher = self
            .event_dispatcher
            .unwrap_or_else(|| Box::<SimpleEventDispatcher>::default());

        Client {
            datafile,
            #[cfg(feature = "online")]
            event_dispatcher,
        }
    }
}
