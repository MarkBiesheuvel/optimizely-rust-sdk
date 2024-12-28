// External imports
use std::sync::mpsc;
use std::thread;

// Imports from super
use super::EventDispatcher;

// Imports from crate
use crate::event_api::request::{Payload, Visitor};
use crate::{client::UserContext, Conversion, Decision};

// Structure used to send message between threads
struct ThreadMessage {
    account_id: String,
    visitor: Visitor,
    event: EventEnum,
}
enum EventEnum {
    Conversion(Conversion),
    Decision(Decision),
}

// Upper limit to number of events in a batch
const DEFAULT_BATCH_THRESHOLD: usize = 10;

/// Implementation of the EventDispatcher trait that collects multiple events before sending them
///
/// TODO: add example usage in SDK
///
/// Inspiration from [Spawn threads and join in destructor](https://users.rust-lang.org/t/spawn-threads-and-join-in-destructor/1613/9)
pub struct BatchedEventDispatcher {
    thread_handle: Option<thread::JoinHandle<()>>,
    transmitter: Option<mpsc::Sender<ThreadMessage>>,
}

impl Default for BatchedEventDispatcher {
    /// Constructor for a new batched event dispatcher
    fn default() -> BatchedEventDispatcher {
        let (transmitter, receiver) = mpsc::channel::<ThreadMessage>();

        // Receiver logic in separate thread
        let thread_handle = thread::spawn(move || {
            let mut payload_option = Option::None;

            // Keep receiving new messages from the main thread
            for message in receiver.iter() {
                // Deconstruct the message
                let ThreadMessage {
                    account_id,
                    visitor,
                    event,
                } = message;

                // Use existing payload or create new one
                let payload = payload_option.get_or_insert_with(|| Payload::new(account_id));

                // the corresponding event to the payload
                match event {
                    EventEnum::Conversion(conversion) => {
                        payload.add_conversion_event(visitor, &conversion);
                    }
                    EventEnum::Decision(decision) => {
                        payload.add_decision_event(visitor, &decision);
                    }
                }

                // Send payload if reached the batch threshold
                if let Some(payload) = payload_option.take_if(|payload| payload.size() >= DEFAULT_BATCH_THRESHOLD) {
                    log::debug!("Reached DEFAULT_BATCH_THRESHOLD");
                    payload.send();
                }
            }
        });

        BatchedEventDispatcher {
            thread_handle: Some(thread_handle),
            transmitter: Some(transmitter),
        }
    }
}

impl Drop for BatchedEventDispatcher {
    fn drop(&mut self) {
        // Take the transmitter_decision and replace it with None
        if let Some(tx) = self.transmitter.take() {
            // Drop the transmitter first, so the receiver in the thread will eventually stop
            drop(tx);
        }

        // Take the thread_handle and replace it with None
        if let Some(handle) = self.thread_handle.take() {
            // Wait until the thread has send the last batch
            let result = handle.join();
            // Ignore result
            drop(result);
        }
    }
}

impl EventDispatcher for BatchedEventDispatcher {
    fn send_conversion_event(&self, user_context: &UserContext, conversion: Conversion) {
        self.transmit(user_context, EventEnum::Conversion(conversion))
    }

    fn send_decision_event(&self, user_context: &UserContext, decision: Decision) {
        self.transmit(user_context, EventEnum::Decision(decision))
    }
}

impl BatchedEventDispatcher {
    fn transmit(&self, user_context: &UserContext, event: EventEnum) {
        // Create a String so the value can be owned by the other thread.
        let account_id = user_context.client().datafile().account_id().into();
        let visitor = Visitor::from(user_context);

        // Build message
        let message = ThreadMessage {
            account_id,
            visitor,
            event,
        };

        // Send message to thread
        match &self.transmitter {
            Some(tx) => match tx.send(message) {
                Ok(_) => {
                    log::debug!("Successfully sent message to thread");
                }
                Err(_) => {
                    log::error!("Failed to send message to thread");
                }
            },
            None => {
                log::error!("Transmitter already dropped");
            }
        }
    }
}
