use axum::{
    extract::State,
    routing::get,
    Router,
};
use std::sync::{Arc, Mutex};

struct AppState {
    counter: usize,
}


async fn handler(State(state): State<Arc<Mutex<AppState>>>) -> String {
    let mut counter = state.lock().unwrap();
    counter.counter += 1;
    format!("Current counter: {}", counter.counter)
}

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(Mutex::new(AppState { counter: 0}) );

    let app = Router::new()
        .route("/", get(handler))
        .with_state(shared_state);
        
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
}
