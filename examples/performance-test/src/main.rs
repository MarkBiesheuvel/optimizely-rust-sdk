use optimizely::{decision::DecideOptions, Client};
use std::error::Error;

const FILE_PATH: &str = "../../datafiles/sandbox.json";
const FLAG_KEY: &str = "buy_button";

fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::from_local_datafile(FILE_PATH)?.initialize();

    // Do not send any decision events during performance testing
    let decide_options = DecideOptions {
        disable_decision_event: true,
        ..DecideOptions::default()
    };

    for i in 0..1_000_000 {
        let user_id = format!("user{}", i);
        let user_context = client.create_user_context(&user_id);
        let _decision = user_context.decide_with_options(FLAG_KEY, &decide_options);
    }

    Ok(())
}
