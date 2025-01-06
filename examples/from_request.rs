use axum::body::Bytes;
use axum::extract::{FromRequest, FromRequestParts, Path, Query, Request};

use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use std::collections::HashMap;

use tokio::net::TcpListener;

// 定义一个结构体`ValidatedBody`，用于封装经过验证的请求体
struct ValidatedBody(Bytes);

// 实现`FromRequest` trait以自定义请求提取逻辑
impl<S> FromRequest<S> for ValidatedBody
where
    S: Send + Sync,
{
    type Rejection = Response;

    // 从请求中提取并验证请求体
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let body = Bytes::from_request(req, state)
            .await
            .map_err(IntoResponse::into_response)?;
        Ok(Self(body))
    }
}

// 处理GET请求的handler函数
async fn handler(
    Query(query): Query<HashMap<String, String>>,
    ValidatedBody(user_agent): ValidatedBody,
) -> Response {
    // 打印请求体和查询参数
    println!("{:?}", user_agent);
    println!("{:?}", query);
    // 返回HTTP 200响应
    (StatusCode::OK, "hello world".to_string()).into_response()
}

// 处理POST请求的handler函数
async fn post_handler(
    Path(path): Path<String>,
    Query(query): Query<HashMap<String, String>>,
    header_map: HeaderMap,
    Json(body): Json<HashMap<String, String>>,
) -> Response {
    // 打印请求体、路径参数、查询参数和请求头
    println!("{:?}", body);
    println!("{:?}", path);
    println!("{:?}", query);
    println!("{:?}", header_map);
    // 返回HTTP 200响应
    (StatusCode::OK, "hello world".to_string()).into_response()
}

#[tokio::main]
async fn main() {
    // 创建路由，同时处理GET和POST请求
    let app = Router::new()
        .route("/{id}", get(handler))
        .route("/{id}", post(post_handler));
    // 绑定TCP监听器
    let tcp = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // 启动HTTP服务器
    axum::serve(tcp, app).await.unwrap()
}
