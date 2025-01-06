use axum::extract::{FromRequestParts, Query}; // 导入 axum 的 FromRequestParts 和 Query 特性
use axum::http::header::USER_AGENT; // 导入 axum 的 USER_AGENT 常量
use axum::http::request::Parts; // 导入 axum 的 Parts 结构体
use axum::http::{HeaderValue, StatusCode}; // 导入 axum 的 HeaderValue 和 StatusCode 类型
use axum::response::{IntoResponse, Response}; // 导入 axum 的 IntoResponse 和 Response 特性
use axum::routing::get; // 导入 axum 的 get 宏
use axum::Router; // 导入 axum 的 Router 结构体
use std::collections::HashMap; // 导入标准库的 HashMap 类型
use tokio::net::TcpListener; // 导入 tokio 的 TcpListener 结构体

/// 结构体 `ExtractUserAgent` 用于从请求头中提取 `USER_AGENT`。
/// 这个结构体将包含一个 `HeaderValue` 类型的字段，该字段存储了从请求头中提取的 `USER_AGENT` 值。
struct ExtractUserAgent(HeaderValue);

/// 实现 `FromRequestParts` trait 以便从请求部分提取 `ExtractUserAgent`。
/// 通过实现这个 trait，我们可以自定义如何从请求部分中提取 `ExtractUserAgent`。
impl<S> FromRequestParts<S> for ExtractUserAgent
where
    S: Send + Sync,
{
    /// 自定义拒绝类型，返回状态码和错误信息。
    /// 当无法从请求头中提取 `USER_AGENT` 时，将返回一个包含状态码和错误信息的元组。
    type Rejection = (StatusCode, &'static str);
    /// 从请求部分异步提取 `USER_AGENT`。
    /// 该方法尝试从请求头中获取 `USER_AGENT`，如果成功则返回 `ExtractUserAgent`，否则返回拒绝类型。
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(user_agent) = parts.headers.get(USER_AGENT) {
            Ok(ExtractUserAgent(user_agent.clone())) // 如果找到了 USER_AGENT，则返回 ExtractUserAgent
        } else {
            Err((StatusCode::BAD_REQUEST, "no user agent")) // 如果没有找到 USER_AGENT，则返回错误
        }
    }
}

/// 处理函数，接收 `ExtractUserAgent` 和查询参数，打印并返回响应。
/// 该函数接收两个参数：`ExtractUserAgent` 和 `Query<HashMap<String, String>>`。
/// 它将打印 `USER_AGENT` 和查询参数，并返回一个包含状态码和响应体的 `Response`。
async fn handler(
    ExtractUserAgent(user_agent): ExtractUserAgent, // 提取 ExtractUserAgent
    Query(query): Query<HashMap<String, String>>, // 提取查询参数
) -> Response {
    println!("{:?}", user_agent); // 打印 USER_AGENT
    println!("{:?}", query); // 打印查询参数
    (StatusCode::OK, "hello world".to_string()).into_response() // 返回响应
}

/// 主函数，设置路由并启动服务器。
/// 在主函数中，我们创建了一个路由并将其绑定到 `handler` 函数，然后启动服务器并监听端口 3000。
#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler)); // 创建路由并绑定 handler 函数
    let tcp = TcpListener::bind("0.0.0.0:3000").await.unwrap(); // 绑定到端口 3000
    axum::serve(tcp, app).await.unwrap() // 启动服务器
}
