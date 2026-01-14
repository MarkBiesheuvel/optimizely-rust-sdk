// Incorrect warnings of dead code: https://github.com/rust-lang/rust/issues/46379
#![allow(dead_code)]

// External imports
use std::error::Error;
use std::sync::{Arc, RwLock};

// Imports from Optimizely crate
use optimizely::{client::UserContext, event_api::EventDispatcher, Client, Conversion, Decision};

// This is the account ID of mark.biesheuvel@optimizely.com
pub const ACCOUNT_ID: &str = "21537940595";

// SDK key for the development environment of mark.biesheuvel@optimizely.com
// This key only grants read access to a JSON file and does not grant any further permissions
pub const SDK_KEY: &str = "UCtKi3qiMkQpso1GTmBFY";

// This is a bundled copy of the JSON file that can be downloaded with the SDK key
pub const FILE_PATH: &str = "../datafiles/sandbox.json";

// This is the revision number of the bundled datafile
pub const REVISION: u32 = 21;

// In-memory thread-safe list of any type
#[derive(Default)]
pub struct Counter(Arc<RwLock<usize>>);

impl Counter {
    fn increment(&self) {
        // Acquire write lock and increment value
        if let Ok(mut lock_guard) = self.0.write() {
            *lock_guard += 1
        }
    }

    pub fn value(&self) -> usize {
        // Acquire read lock and return value or 0
        self.0.read().map(|value| *value).unwrap_or_default()
    }

    fn clone(&self) -> Self {
        // Clone the atomic counted reference
        Self(self.0.clone())
    }
}

// Struct that holds conversion and decisions in memory and implement the EventDispatcher trait
#[derive(Default)]
pub struct EventStore {
    conversion_counter: Counter,
    decision_counter: Counter,
}

// Implementing the EventDispatcher using the interior mutability pattern
impl EventDispatcher for EventStore {
    fn send_conversion_event(&self, _user_context: &UserContext, _conversion: &Conversion) {
        self.conversion_counter.increment();
    }

    fn send_decision_event(&self, _user_context: &UserContext, _decision: &Decision) {
        self.decision_counter.increment();
    }
}

// Return struct from setup function that contains:
// - an Optimizely client
// - a list of events that was send to the EventDispatcher
pub struct TestContext {
    pub client: Client,
    pub conversion_counter: Counter,
    pub decision_counter: Counter,
}

// A setup function used in multiple tests
pub fn setup() -> Result<TestContext, Box<dyn Error>> {
    // Create event store
    let event_store = EventStore::default();

    // Clone the counters
    let conversion_counter = event_store.conversion_counter.clone();
    let decision_counter = event_store.decision_counter.clone();

    // Build client
    let client = Client::from_local_datafile(FILE_PATH)?
        .with_event_dispatcher(|_datafile| event_store)
        .initialize();

    Ok(TestContext {
        client,
        conversion_counter,
        decision_counter,
    })
}
