use axum::{
    extract::State,
    routing::get,
    Router,
};
use std::sync::{Arc, Mutex};

struct AppState {
    counter: Mutex<usize>,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            counter: Mutex::new(*self.counter.lock().unwrap()),
        }
    }
}

async fn handler(State(state): State<Arc<AppState>>) -> String {
    let mut counter = state.counter.lock().unwrap();
    *counter += 1;
    format!("Current counter: {}", *counter)
}

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(AppState { counter: Mutex::new(0) });

    let app = Router::new()
        .route("/", get(handler))
        .with_state(shared_state);
        
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
}
