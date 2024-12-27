use optimizely::{decision::DecideOptions, Client};
use std::{collections::HashMap, error::Error, ops::AddAssign};
use uuid::Uuid;

const FILE_PATH: &str = "../../datafiles/sandbox.json";
const FLAG_KEY: &str = "buy_button";

fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::from_local_datafile(FILE_PATH)?.initialize();

    // Do not send any decision events during SRM testing
    let decide_options = DecideOptions {
        disable_decision_event: true,
        ..DecideOptions::default()
    };

    // Create counter
    let mut counter: HashMap<String, usize> = HashMap::new();

    // Run a two million times
    for _ in 0..2_000_000 {
        // Generate visitor ID using UUIDv4
        let user_id = Uuid::new_v4().as_hyphenated().to_string();

        // Get variation key
        let variation_key = client
            .create_user_context(&user_id)
            .decide_with_options(FLAG_KEY, &decide_options)
            .variation_key()
            .to_string();

        // Count variation
        counter.entry(variation_key).or_insert(0).add_assign(1);
    }

    // Dump counter to stdout
    for (key, total) in counter {
        println!("Variation '{}' had {} visitors", key, total);
    }

    Ok(())
}
