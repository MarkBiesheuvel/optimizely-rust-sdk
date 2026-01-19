use axum::{
    extract::State,
    response::{Html, Redirect},
    routing::get,
    Router,
};
use env_logger::Target;
use log::LevelFilter;
use optimizely::{Client, DecideOptions};
use std::{
    sync::{Arc, RwLock},
    time::Duration,
};
use uuid::Uuid;

const SDK_KEY: &str = "KVpGWnzPGKvvQ8yeEWmJZ";
const FLAG_KEY: &str = "issue_23";

#[derive(Clone)]
struct AppState {
    client: Arc<Client>,
    user_id: Arc<RwLock<String>>,
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

    // Generate user ID
    let user_id = Uuid::new_v4().as_hyphenated().to_string();

    // Initialize state with client and potential other properties
    let state = Arc::new(AppState {
        client: Arc::new(client),
        user_id: Arc::new(RwLock::new(user_id)),
    });

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/new", get(new_user_id))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler(State(state): State<Arc<AppState>>) -> Html<String> {
    // Get user ID from state
    let user_id = state.user_id.read().unwrap();
    // Create user context
    let user_context = state.client.create_user_context(&user_id);
    // Decide variation for user
    let decision = user_context.decide(FLAG_KEY);
    // Extract variation key
    let variation_key = decision.variation_key();
    // Generate HTML
    let body = format!("<h1>Hello, <code>{user_id}</code>!</h1><p>Variation key: <code>{variation_key}</code></p><a href='/'>Refresh</a> <a href='/new'>Get new user ID.</a>");
    // Return
    Html(body)
}

async fn new_user_id(State(state): State<Arc<AppState>>) -> Redirect {
    let mut write_lock = state.user_id.write().unwrap();
    // Generate user ID
    *write_lock = Uuid::new_v4().as_hyphenated().to_string();

    Redirect::temporary("/")
}
