use crate::common::{self, get_global_map, PRIVATE_KEY_STR};
use axum::{extract::Query, http::StatusCode, response::IntoResponse, Json};
use log::{error, info};
use rust_utils::decrypt_data;
use rust_utils::gen_rand;
use serde::{Deserialize, Serialize};
use std::time::Duration;

// 定义登录请求和响应结构
#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    nick_name: String,
    auth_key_enc: String,
}

// 定义响应结构体
#[derive(Debug, Serialize)]
#[serde(untagged)] // 避免嵌套标签
enum AuthResponse {
    Success { token: String },
    Error { err_no: u32 },
}

pub async fn handler_auth(Json(payload): Json<AuthRequest>) -> impl IntoResponse {
    // info!("nick name:{}", payload.nick_name);
    // info!("auth_key_enc:{}", payload.auth_key_enc);

    // 模拟一些较长的操作
    // tokio::time::sleep(Duration::from_secs(35)).await;

    let decrypted_data =
        decrypt_data(PRIVATE_KEY_STR, payload.auth_key_enc.as_str()).unwrap_or("".to_string());

    info!("[auth] login user:{}", payload.nick_name);
    info!("[auth] auth key from agent:{}", decrypted_data.as_str());

    let auth_key = common::get_auth_key();

    if decrypted_data.as_str() == auth_key {
        let token = gen_rand(6);
        info!("[auth] auth ok.token:{}", token);

        // 保存token，过期时间：1小时
        let map = get_global_map();
        map.set(
            token.clone(),
            payload.nick_name,
            Some(Duration::from_secs(60 * 60)),
        );

        // 成功返回 token
        (StatusCode::OK, Json(AuthResponse::Success { token }))
    } else {
        error!("[auth] auth failed.");
        // 用户名或密码错误
        (
            StatusCode::UNAUTHORIZED,
            Json(AuthResponse::Error { err_no: 101 }),
        )
    }
}

// pub async fn handler_auth(Query(params): Query<AuthRequest>) -> impl IntoResponse {
//     // 模拟逻辑判断
//     if let (Some(nick_name), Some(encrypt_str)) = (params.nick_name, params.auth_key_enc) {
//         let decrypted_data =
//             decrypt_data(PRIVATE_KEY_STR, encrypt_str.as_str()).unwrap_or("".to_string());

//         info!("[auth] login user:{}", nick_name);
//         info!("[auth] auth key from agent:{}", decrypted_data.as_str());

//         let auth_key = common::get_auth_key();

//         if decrypted_data.as_str() == auth_key {
//             info!("[auth] auth ok.");
//             // 成功返回 token
//             (
//                 StatusCode::OK,
//                 Json(AuthResponse::Success {
//                     token: "example_token_123".to_string(),
//                 }),
//             )
//         } else {
//             error!("[auth] auth failed.");
//             // 用户名或密码错误
//             (
//                 StatusCode::UNAUTHORIZED,
//                 Json(AuthResponse::Error { err_no: 101 }),
//             )
//         }
//     } else {
//         error!("[auth] bad request.");
//         // 缺少必要参数
//         // Json(AuthResponse::Error { err_no: 100 })
//         (
//             StatusCode::BAD_REQUEST,
//             Json(AuthResponse::Error { err_no: 100 }),
//         )
//     }
// }
