use env_logger::Target;
use log::LevelFilter;
use optimizely::{event_api::BatchedEventDispatcher, Client};
use std::error::Error;

const SDK_KEY: &str = "KVpGWnzPGKvvQ8yeEWmJZ";

fn main() -> Result<(), Box<dyn Error>> {
    // Always set log level to debug
    env_logger::builder()
        .target(Target::Stdout)
        .filter_module("optimizely", LevelFilter::Debug)
        .init();

    // Initiate client using SDK key and batched event dispatcher
    let client = Client::from_sdk_key(SDK_KEY)?
        .with_event_dispatcher(BatchedEventDispatcher::default())
        .initialize();

    let flag_key = "buy_button";

    for i in 0..20 {
        let user_id = format!("user{}", i);
        let user_context = client.create_user_context(&user_id);
        let _decision = user_context.decide(flag_key);
    }

    Ok(())
}
