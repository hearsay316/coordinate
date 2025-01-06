use axum::extract::{Multipart, OriginalUri, Query};
use axum::http::{HeaderMap, HeaderValue, Method, StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::{extract::Path, routing::get, Router};
use std::collections::HashMap;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

async fn users_get(
    uri: Uri,
    OriginalUri(original_uri): OriginalUri,
    Path(params): Path<HashMap<String, String>>,
    headers: HeaderMap,
    Query(query): Query<HashMap<String, String>>,
) -> String {
    // Both `version` and `id` were captured even though `users_api` only
    // explicitly captures `id`.
    println!("{:?}", uri);
    println!("{:?}", original_uri);
    println!("{:?}", params);
    println!("{:?}", headers);
    println!("{:?}", query);
    "这个是一个请求".to_string()
}
//serde::json serde_json 给他传的是什么 引用  // 不要要实例  引用切片&str  &bytes
async fn accept_form(headers: HeaderMap, mut multipart: Multipart) -> Response {
    println!("{:?}", headers);
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let file_name = field.file_name().unwrap_or("").to_string();
        let content_type = field.content_type().unwrap_or("").to_string();
        if name == "text" {
            // 保存文件
            let text = field.text().await.unwrap();
            println!("Length of  (`{name}`: `{text}`) ",);
        } else {
            let data = field.bytes().await.unwrap();
            println!(
                "Length of `{name}` (`{file_name}`: `{content_type}`) is {} bytes",
                data.len()
            );
        }
    }
    (StatusCode::OK, "上传成功".to_string()).into_response()
}
#[tokio::main]
async fn main() {
    let users_api = Router::new().route("/users/{id}", get(users_get).post(accept_form));
    let app = Router::new().nest("/{version}/api", users_api).layer(
        // see https://docs.rs/tower-http/latest/tower_http/cors/index.html
        // for more details
        //
        // pay attention that for some request types like posting content-type: application/json
        // it is required to add ".allow_headers([http::header::CONTENT_TYPE])"
        // or see this issue https://github.com/tokio-rs/axum/issues/849
        CorsLayer::new()
            .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
            .allow_methods([Method::GET, Method::POST]),
    );
    let tcp = TcpListener::bind("0.0.0.0:3303").await.unwrap();
    axum::serve(tcp, app).await.unwrap();
}
