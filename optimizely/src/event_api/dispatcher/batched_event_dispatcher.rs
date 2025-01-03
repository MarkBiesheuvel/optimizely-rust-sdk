// External imports
use std::sync::mpsc;
use std::thread;

// Imports from super
use super::EventDispatcher;

// Imports from crate
use crate::event_api::request::{Request, Visitor};
use crate::{client::UserContext, datafile::Datafile, Conversion, Decision};

// Structure used to send message between threads
struct ThreadMessage {
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
    receiver_thread: Option<thread::JoinHandle<()>>,
    transmitter_channel: Option<mpsc::Sender<ThreadMessage>>,
}

impl BatchedEventDispatcher {
    /// Constructor for a new batched event dispatcher
    pub fn new(datafile: &Datafile) -> Self {
        // Create the request buffer using the datafile
        let mut request = Request::new(datafile);

        // Create sender and receiver for thread
        let (transmitter_channel, receiver_channel) = mpsc::channel::<ThreadMessage>();

        // Receiver logic in separate thread
        let receiver_thread = thread::spawn(move || {
            // Keep receiving new messages from the main thread
            for message in receiver_channel.iter() {
                // Deconstruct the message
                let ThreadMessage { visitor, event } = message;

                // the corresponding event to the payload
                match event {
                    EventEnum::Conversion(conversion) => {
                        request.add_conversion_event(visitor, &conversion);
                    }
                    EventEnum::Decision(decision) => {
                        request.add_decision_event(visitor, &decision);
                    }
                }

                // Send request if reached the batch threshold
                if request.buffer_size() >= DEFAULT_BATCH_THRESHOLD {
                    log::debug!("Reached DEFAULT_BATCH_THRESHOLD");

                    // Sending request
                    request.send();
                }
            }
        });

        BatchedEventDispatcher {
            receiver_thread: Some(receiver_thread),
            transmitter_channel: Some(transmitter_channel),
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

impl Drop for BatchedEventDispatcher {
    fn drop(&mut self) {
        // Take the transmitter channel and replace it with None
        if let Some(channel) = self.transmitter_channel.take() {
            // Drop the transmitter channel first, so the receiver channel in the thread will stop receiving messages
            drop(channel);
        }

        // Take the receiver thread and replace it with None
        if let Some(thread) = self.receiver_thread.take() {
            // Wait until the thread has send the last batch
            let result = thread.join();

            // Log thread error
            if result.is_err() {
                log::error!("Failed to wait for receiver thread");
            }
        }
    }
}

impl BatchedEventDispatcher {
    fn transmit(&self, user_context: &UserContext, event: EventEnum) {
        // Create a String so the value can be owned by the other thread.
        let visitor = Visitor::from(user_context);

        // Build message
        let message = ThreadMessage { visitor, event };

        // Send message to thread
        match &self.transmitter_channel {
            Some(channel) => match channel.send(message) {
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
