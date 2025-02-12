// use axum::{
//     http::StatusCode,
//     response::IntoResponse,
//     routing::{get, MethodRouter},
//     Router,
// };
// // use log::info;
// use crate::qc_web::handler_auth;
// use std::collections::HashMap;
// use std::error::Error;

use crate::qc_web::handler_auth;
use axum::{
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, MethodRouter},
    Router,
};
use log::info;
// use serde_urlencoded;
use std::collections::HashMap;
use std::error::Error;

pub struct RouteRegistry {
    routes: HashMap<&'static str, MethodRouter>,
}
impl RouteRegistry {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }
    pub fn register(&mut self, path: &'static str, method: MethodRouter) {
        self.routes.insert(path, method);
    }
    pub fn build_app(&self) -> Router {
        let mut router = Router::new();
        for (path, method) in &self.routes {
            router = router.route(path, method.clone());
        }
        router.layer(middleware::from_fn(axum_middleware))
    }
}
async fn handler_index() -> impl IntoResponse {
    (StatusCode::OK, "Welcome to the Index Page!")
}
async fn handler_fallback() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "404 Not Found")
}

pub async fn start_web_server(ip: &str, port: u16) -> Result<(), Box<dyn Error>> {
    // 创建路由注册器
    let mut registry = RouteRegistry::new();

    // 注册路由
    registry.register("/index.html", get(handler_index));
    registry.register("/auth", get(handler_auth::handler_auth));

    // 构建应用路由
    let app = registry.build_app().fallback(handler_fallback);

    // let add_str = format!("{}{}", ip, port);
    // 启动服务
    let addr = format!("{}:{}", ip, port).parse()?;
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn axum_middleware<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode>
where
    B: Send + 'static, // 保证 B 是可发送的
{
    // 获取请求的 URL
    let uri = req.uri().to_string();

    // 提取查询参数
    // if let Some(query) = req.uri().query() {
    // let query_params: HashMap<String, String> =
    //     serde_urlencoded::from_str(query).unwrap_or_default();

    // 将 uri 和 query_params 打印在同一行
    // info!("Request URL: {} Query parameters: {:?}", uri, query_params);
    //     info!("Request URL: {} ", uri);
    // } else {
    //     info!("Request URL: {} ", uri);
    // }

    info!("Request URL: {} ", uri);

    // 继续执行请求处理
    Ok(next.run(req).await)
}
