//! Entrypoint of the SDK

// Imports from crate
use crate::datafile::Datafile;
#[cfg(feature = "online")]
use crate::event_api::EventDispatcher;

// Relative imports of sub modules
pub use error::ClientError;
pub use initialization::UninitializedClient;
pub use user::{UserAttributes, UserContext};

mod error;
mod initialization;
mod user;

/// SDK client to use Optimizely Feature Experimentation
///
/// ```
/// use optimizely::Client;
/// #
/// # let file_path = "../datafiles/sandbox.json";
/// # let user_id = "123abc789xyz";
///
/// // Initialize Optimizely client using local datafile
/// let optimizely_client = Client::from_local_datafile(file_path)?
///     .initialize();
///
/// // Use methods of client struct
/// let account_id = optimizely_client.datafile().account_id();
/// let revision = optimizely_client.datafile().revision();
/// let user_context = optimizely_client.create_user_context(user_id);
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct Client {
    datafile: Datafile,
    #[cfg(feature = "online")]
    event_dispatcher: Box<dyn EventDispatcher>,
}

impl Client {
    /// Create a new user context for a given user id
    pub fn create_user_context<'a>(&'a self, user_id: &'a str) -> UserContext<'a> {
        // Create an empty set of user attributes
        let attributes = UserAttributes::new();

        UserContext::new(self, user_id, attributes)
    }

    /// Create a new user context for a given user id
    pub fn create_user_context_with_attributes<'a>(
        &'a self, user_id: &'a str, attributes: UserAttributes,
    ) -> UserContext<'a> {
        UserContext::new(self, user_id, attributes)
    }

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
