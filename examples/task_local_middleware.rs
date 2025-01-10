use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use tokio::net::TcpListener;
use tokio::task_local;

// 定义一个结构体来表示当前用户
#[derive(Clone)]
struct CurrentUser {
    name: String,
}

// 使用 task_local! 宏创建一个静态的、线程局部的存储，用于存储当前用户
task_local! {
    pub static USER: CurrentUser;
}

// 认证中间件函数，用于检查请求头中的 Authorization 字段，并设置当前用户
async fn auth(req: Request, next: Next) -> Result<Response, StatusCode> {
    // 从请求头中获取 Authorization 字段
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    // 调用 authorize_current_user 函数来验证用户，如果验证通过，则设置当前用户
    if let Some(current_user) = authorize_current_user(auth_header).await {
        // 在中间件中设置当前用户的状态
        Ok(USER.scope(current_user, next.run(req)).await)
    } else {
        // 如果用户未通过认证，则返回 401 Unauthorized 状态码
        Err(StatusCode::UNAUTHORIZED)
    }
}

// 模拟一个函数来验证当前用户，这里只是简单地根据 auth_token 返回一个 CurrentUser 实例
async fn authorize_current_user(auth_token: &str) -> Option<CurrentUser> {
    Some(CurrentUser {
        name: auth_token.to_string(),
    })
}

// 定义一个结构体来表示用户响应
struct UserResponse;

// 实现 IntoResponse trait，用于将 UserResponse 转换为 HTTP 响应
impl IntoResponse for UserResponse {
    fn into_response(self) -> Response {
        // 在响应处理中访问当前用户的状态
        let current_user = USER.with(|u| u.clone());
        // 返回一个包含当前用户名字的 200 OK 响应
        (StatusCode::OK, current_user.name).into_response()
    }
}

// 处理函数，返回一个 UserResponse 实例
async fn handler() -> UserResponse {
    UserResponse
}

// 主函数，用于启动 Web 服务
#[tokio::main]
async fn main() {
    // 创建一个路由器，并定义一个 GET 请求处理函数
    let app: Router = Router::new()
        .route("/", get(handler))
        // 应用认证中间件到所有路由
        .route_layer(middleware::from_fn(auth));
    // 绑定到本地的 TCP 端口 3000
    let tcp = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    // 使用 axum 启动服务
    axum::serve(tcp, app).await.unwrap();
}
