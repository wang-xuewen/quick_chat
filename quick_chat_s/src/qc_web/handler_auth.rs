use axum::{extract::Query, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

// 定义登录请求和响应结构
#[derive(Deserialize)]
pub struct AuthRequest {
    nick_name: Option<String>,
    encrypt_str: Option<String>,
}

// 定义响应结构体
#[derive(Serialize)]
#[serde(untagged)] // 避免嵌套标签
enum AuthResponse {
    Success { token: String },
    Error { err_no: u32 },
}

pub async fn handler_auth(Query(params): Query<AuthRequest>) -> impl IntoResponse {
    // 模拟逻辑判断
    if let (Some(nick_name), Some(encrypt_str)) = (params.nick_name, params.encrypt_str) {
        if nick_name == "admin" && encrypt_str == "password" {
            // 成功返回 token
            Json(AuthResponse::Success {
                token: "example_token_123".to_string(),
            })
        } else {
            // 用户名或密码错误
            Json(AuthResponse::Error { err_no: 101 })
        }
    } else {
        // 缺少必要参数
        Json(AuthResponse::Error { err_no: 100 })
    }
}
