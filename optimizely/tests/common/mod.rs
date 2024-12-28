// Incorrect warnings of dead code: https://github.com/rust-lang/rust/issues/46379
#![allow(dead_code)]

// External imports
use std::sync::{Arc, RwLock};

// Imports from Optimizely crate
use optimizely::{client::UserContext, event_api::EventDispatcher, Client, Conversion, Decision};

// This is the account ID of mark.biesheuvel@optimizely.com
pub const ACCOUNT_ID: &str = "21537940595";

// SDK key for the development environment of mark.biesheuvel@optimizely.com
// This key only grants read access to a JSON file and does not grant any further permissions
pub const SDK_KEY: &str = "KVpGWnzPGKvvQ8yeEWmJZ";

// This is a bundled copy of the JSON file that can be downloaded with the SDK key
pub const FILE_PATH: &str = "../datafiles/sandbox.json";

// This is the revision number of the bundled datafile
pub const REVISION: u32 = 544;

// In-memory thread-safe list of any type
pub struct SyncList<T>(Arc<RwLock<Vec<T>>>);

impl<T> Default for SyncList<T> {
    fn default() -> Self {
        Self(Arc::new(RwLock::new(Vec::default())))
    }
}

impl<T> SyncList<T> {
    fn add(&self, item: T) {
        // Acquire lock on the RwLock
        if let Ok(mut vec) = self.0.write() {
            // Add item to the list
            vec.push(item);
        } else {
            // Error handling not implemented in this example
        }
    }

    pub fn len(&self) -> usize {
        match self.0.read() {
            Ok(vec) => vec.len(),
            Err(_) => 0,
        }
    }

    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

// Struct that holds conversion and decisions in memory and implement the EventDispatcher trait
#[derive(Default)]
pub struct EventStore {
    conversions: SyncList<Conversion>,
    decisions: SyncList<Decision>,
}

// Implementing the EventDispatcher using the interior mutability pattern
impl EventDispatcher for EventStore {
    fn send_conversion_event(&self, _user_context: &UserContext, conversion: Conversion) {
        self.conversions.add(conversion);
    }

    fn send_decision_event(&self, _user_context: &UserContext, decision: Decision) {
        self.decisions.add(decision);
    }
}

// Return struct from setup function that contains:
// - an Optimizely client
// - a list of events that was send to the EventDispatcher
pub struct TestContext {
    pub client: Client,
    pub conversions: SyncList<Conversion>,
    pub decisions: SyncList<Decision>,
}

// A setup function used in multiple tests
pub fn setup() -> TestContext {
    // Create a struct to store events
    let event_store = EventStore::default();

    // Clone reference to the lists
    let conversions = event_store.conversions.clone();
    let decisions = event_store.decisions.clone();

    // Build client
    let client = Client::from_local_datafile(FILE_PATH)
        .expect("local datafile should work")
        .with_event_dispatcher(event_store)
        .initialize();

    TestContext {
        client,
        conversions,
        decisions,
    }
}
