use crate::common::{self, PUBLIC_KEY_STR};
use anyhow::{anyhow, Result};
// use reqwest::Error;
use log::info;
use rust_utils::encrypt_data;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    token: String,
}

#[derive(Debug, Serialize)]
struct ApiRequest {
    nick_name: String,
    auth_key_enc: String,
}

pub async fn call_auth(nick_name: &str, auth_key: &str) -> Result<String> {
    info!("call auth.");

    let auth_key_enc = encrypt_data(PUBLIC_KEY_STR, auth_key).unwrap_or("".to_string());
    if auth_key_enc == "" {
        return Err(anyhow!("encryption auth key failed."));
    }

    // 创建请求体
    let request_body = ApiRequest {
        nick_name: nick_name.to_string(),
        auth_key_enc,
    };

    // 发送 POST 请求
    let client = reqwest::Client::new();
    let response = client
        .post("http://127.0.0.1:8080/auth")
        .json(&request_body)
        .timeout(Duration::from_secs(30)) // 设置超时时间为 30 秒
        .send()
        .await?;

    // 解析 JSON 响应
    let api_response: ApiResponse = response.json().await?;

    info!("call auth ok.");
    // 返回 token
    Ok(api_response.token)
}
