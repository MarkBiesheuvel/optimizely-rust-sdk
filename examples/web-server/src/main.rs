use axum::{extract::State, response::Html, routing::get, Router};
use env_logger::Target;
use log::LevelFilter;
use optimizely::{decision::DecideOptions, Client};
use std::{sync::Arc, time::Duration};
use uuid::Uuid;

const SDK_KEY: &str = "KVpGWnzPGKvvQ8yeEWmJZ";
const FLAG_KEY: &str = "issue_23";

#[derive(Clone)]
struct AppState {
    client: Arc<Client>,
}

#[tokio::main]
async fn main() {
    // Set log level to debug
    env_logger::builder()
        .target(Target::Stdout)
        .filter_module("optimizely", LevelFilter::Info)
        .init();

    // Do not send any decision events
    let decide_options = DecideOptions {
        disable_decision_event: true,
        ..DecideOptions::default()
    };

    // Initiate client using SDK key and batched event dispatcher
    let client = Client::from_sdk_key(SDK_KEY)
        .expect("Client should initialize with SDK key")
        .with_default_decide_options(decide_options)
        .with_update_interval(Duration::from_secs(5))
        .initialize();

    // Initialize state with client and potential other properties
    let state = AppState {
        client: Arc::new(client),
    };

    // build our application with a route
    let app = Router::new().route("/", get(handler)).with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler(State(state): State<AppState>) -> Html<String> {
    // Generate user ID
    let user_id = Uuid::new_v4().as_hyphenated().to_string();
    // Create user context
    let user_context = state.client.create_user_context(&user_id);
    // Decide variation for user
    let decision = user_context.decide(FLAG_KEY);
    // Extract variation key
    let variation_key = decision.variation_key();
    // Generate HTML
    let body = format!("<h1>Hello, <code>{user_id}</code>!</h1><p>Variation key: <code>{variation_key}</code></p>");
    // Return
    Html(body)
}
