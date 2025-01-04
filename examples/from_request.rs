use axum::body::Bytes;
use axum::extract::{FromRequest, FromRequestParts, Path, Query, Request};

use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use std::collections::HashMap;

use tokio::net::TcpListener;

struct ValidatedBody(Bytes);
impl<S> FromRequest<S> for ValidatedBody
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let body = Bytes::from_request(req, state)
            .await
            .map_err(IntoResponse::into_response)?;
        Ok(Self(body))
    }
}

async fn handler(
    Query(query): Query<HashMap<String, String>>,

    ValidatedBody(user_agent): ValidatedBody,
) -> Response {
    println!("{:?}", user_agent);
    println!("{:?}", query);
    (StatusCode::OK, "hello world".to_string()).into_response()
}
async fn post_handler(Path(path):Path<String>,Query(query): Query<HashMap<String, String>>,
                      header_map: HeaderMap,
                      Json(body): Json<HashMap<String, String>>) -> Response {
    println!("{:?}", body);
    println!("{:?}", path);
    println!("{:?}", query);
    println!("{:?}", header_map);
    (StatusCode::OK, "hello world".to_string()).into_response()
}
#[tokio::main]
async fn main() {
    let app = Router::new().route("/{id}", get(handler)).route("/{id}",post(post_handler));
    let tcp = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(tcp, app).await.unwrap()
}
// postman post 请求  request.get("http://) 前置要求的 我要看前置条件 复杂不复杂 ,变通的和人沟通
