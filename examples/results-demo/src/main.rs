use optimizely::{event_api::BatchedEventDispatcher, Client};
use rand::random;
use std::collections::HashMap;
use std::error::Error;
use std::thread::sleep;
use std::time::{Duration, Instant};
use uuid::Uuid;

// Production SDK key
const SDK_KEY: &str = "4E5fffEkXpkgULW9AhtCL";

// Flag for which to generate data
const FLAG_KEY: &str = "results_demo";

// Event for which to generate conversions
const ADD_TO_CART_EVENT_KEY: &str = "add_to_cart";
const PURCHASE_EVENT_KEY: &str = "purchase";

/// Whether a random event does or doesn't happen
fn random_event_does_happen(chance: f32) -> bool {
    random::<f32>() < chance
}

/// Random revenue value between $0.00 and $655.35
fn random_revenue() -> String {
    random::<u16>().to_string()
}

fn random_category() -> &'static str {
    match random::<f32>() {
        x if x < 0.4 => "games",
        x if x < 0.7 => "controllers",
        x if x < 0.8 => "headsets",
        x if x < 0.9 => "mouses",
        _ => "keyboards",
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Initiate client using SDK key and batched event dispatcher
    let client = Client::from_sdk_key(SDK_KEY)?
        .with_event_dispatcher(BatchedEventDispatcher::new)
        .initialize();

    // Not super accurate interval, but fine for this example
    let interval = Duration::from_millis(166);
    let mut next_iteration = Instant::now();
    loop {
        // There are 10 users in a batch, so it makes sense to sleep between each 10
        for _ in 0..10 {
            // Generate a random user
            let user_id = Uuid::new_v4().as_hyphenated().to_string();
            let user_context = client.create_user_context(&user_id);

            // Get the Optimizely decision for that user
            let decision = user_context.decide(FLAG_KEY);

            // Different conversion rate depending on variation
            let conversion_rate = match decision.variation_key() {
                "variation_1" => 0.12,
                "variation_2" => 0.14,
                "variation_3" => 0.11,
                _ => 0.0,
            };

            // Random chance that user makes a purchase
            if random_event_does_happen(conversion_rate) {
                let category = random_category();
                let properties = HashMap::from([(String::from("Category"), String::from(category))]);
                let tags = HashMap::default();

                user_context.track_event_with_properties_and_tags(ADD_TO_CART_EVENT_KEY, properties, tags);

                // Purchase change stays equal, 30% change if someone adds something to cart
                if random_event_does_happen(0.3) {
                    let properties = HashMap::from([(String::from("Category"), String::from(category))]);
                    let tags = HashMap::from([(String::from("revenue"), random_revenue())]);

                    user_context.track_event_with_properties_and_tags(PURCHASE_EVENT_KEY, properties, tags);

                    println!("{} {} [add_to_cart + revenue]", &user_id, decision.variation_key());
                } else {
                    println!("{} {} [add_to_cart] ", &user_id, decision.variation_key());
                }
            } else {
                println!("{} {} [no conversion]", &user_id, decision.variation_key());
            }
        }

        // Wait a bit until next batch of users
        next_iteration += interval;
        sleep(next_iteration - Instant::now());
    }
}
