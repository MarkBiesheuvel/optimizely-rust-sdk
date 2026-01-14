use env_logger::Target;
use log::LevelFilter;
use optimizely::{event_api::BatchedEventDispatcher, Client, AttributeValue};
use std::error::Error;
use std::{thread, time};

const SDK_KEY: &str = "KVpGWnzPGKvvQ8yeEWmJZ";

fn main() -> Result<(), Box<dyn Error>> {
    // Always set log level to debug
    env_logger::builder()
        .target(Target::Stdout)
        .filter_module("optimizely", LevelFilter::Debug)
        .init();

    // Initiate client using SDK key and batched event dispatcher
    let client = Client::from_sdk_key(SDK_KEY)?
        .with_event_dispatcher(BatchedEventDispatcher::new)
        .initialize();

    let flag_key = "buy_button";

    // Time between iteration
    let duration = time::Duration::from_millis(100);

    for i in 0..24 {
        let user_id = format!("user{}", i);
        let mut user_context = client.create_user_context(&user_id);
        user_context.set_attribute("app_version", AttributeValue::String("0.5.0".into()));
        user_context.set_attribute("country", AttributeValue::String("nl".into()));
        let decision = user_context.decide(flag_key);
        thread::sleep(duration);
        drop(decision);
    }

    Ok(())
}
