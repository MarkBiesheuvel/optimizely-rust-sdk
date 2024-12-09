// Incorrect warnings of dead code: https://github.com/rust-lang/rust/issues/46379
#![allow(dead_code)]

// External imports
use std::cell::RefCell;
use std::rc::Rc;

// Imports from Optimizely crate
use optimizely::{Client, client::UserContext, Conversion, Decision, event_api::EventDispatcher};

// This is the account ID of mark.biesheuvel@optimizely.com
pub const ACCOUNT_ID: &str = "21537940595";

// SDK key for the development environment of mark.biesheuvel@optimizely.com
// This key only grants read access to a JSON file and does not grant any further permissions
pub const SDK_KEY: &str = "KVpGWnzPGKvvQ8yeEWmJZ";

// This is a bundled copy of the JSON file that can be downloaded with the SDK key
pub const FILE_PATH: &str = "../datafiles/sandbox.json";

// This is the revision number of the bundled datafile
pub const REVISION: u32 = 73;

// List of conversions wrapped in a reference counted mutable memory location
type ConversionList = Rc<RefCell<Vec<Conversion>>>;

// List of decisions wrapped in a reference counted mutable memory location
type DecisionList = Rc<RefCell<Vec<Decision>>>;

// Struct that holds the EventList and implement the EventDispatcher trait
#[derive(Default)]
pub(super) struct EventStore {
    conversions: ConversionList,
    decisions: DecisionList,
}

// Implement Send and Sync for EventStore
unsafe impl Send for EventStore {}
unsafe impl Sync for EventStore {}

// Return a new reference counted point to the list
impl EventStore {
    fn conversions(&self) -> ConversionList {
        Rc::clone(&self.conversions)
    }

    fn decisions(&self) -> DecisionList {
        Rc::clone(&self.decisions)
    }
}

// Implementing the EventDispatcher using the interior mutability pattern
impl EventDispatcher for EventStore {
    fn send_conversion_event(&self, _user_context: &UserContext, conversion: Conversion){
        self.conversions.borrow_mut().push(conversion);
    }
    fn send_decision_event(&self, _user_context: &UserContext, decision: Decision) {
        self.decisions.borrow_mut().push(decision);
    }
}

// Return struct from setup function that contains:
// - an Optimizely client
// - a list of events that was send to the EventDispatcher
pub struct TestContext {
    pub client: Client,
    pub conversions: ConversionList,
    pub decisions: DecisionList,
}

// A setup function used in multiple tests
pub(super) fn setup() -> TestContext {
    // Create a struct to store events
    let event_store = EventStore::default();

    // Clone RC
    let conversions = event_store.conversions();
    let decisions = event_store.decisions();

    // Build client
    let client = Client::from_local_datafile(FILE_PATH)
        .expect("local datafile should work")
        .with_event_dispatcher(event_store)
        .initialize();

    TestContext { client, conversions, decisions }
}
