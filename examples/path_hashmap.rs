use axum::{
    extract::Path,
    routing::get,
    Router,
};
use std::collections::HashMap;
use tokio::net::TcpListener;

async fn users_get(Path(params): Path<HashMap<String, String>>)->String {
    // Both `version` and `id` were captured even though `users_api` only
    // explicitly captures `id`.
    let version = params.get("version");
    let id = params.get("id");
    println!("{:?}",params);
    "这个是一个请求".to_string()
}
#[tokio::main]
async fn main(){
    let users_api = Router::new().route("/users/{id}", get(users_get));

    let app = Router::new().nest("/{version}/api", users_api);
    let tcp = TcpListener::bind("0.0.0.0:3303").await.unwrap();
    axum::serve(tcp,app).await.unwrap();
}
