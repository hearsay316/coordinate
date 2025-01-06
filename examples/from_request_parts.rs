use axum::extract::{FromRequestParts, Query};
use axum::http::header::USER_AGENT;
use axum::http::request::Parts;
use axum::http::{HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use std::collections::HashMap;
use tokio::net::TcpListener;

struct ExtractUserAgent(HeaderValue);

impl<S> FromRequestParts<S> for ExtractUserAgent
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(user_ageent) = parts.headers.get(USER_AGENT) {
            Ok(ExtractUserAgent(user_ageent.clone()))
        } else {
            Err((StatusCode::BAD_REQUEST, "no user ageent"))
        }
    }
}

async fn handler(
    ExtractUserAgent(user_agent): ExtractUserAgent,
    Query(query): Query<HashMap<String, String>>,
) -> Response {
    println!("{:?}", user_agent);
    println!("{:?}", query);
    (StatusCode::OK, "hello world".to_string()).into_response()
}
#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));
    let tcp = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(tcp, app).await.unwrap()
}
