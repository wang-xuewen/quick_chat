use log::error;
use std::error::Error;
use std::sync::OnceLock;
// 定义全局变量
static HOST_IP: OnceLock<String> = OnceLock::new();
static AUTH_KEY: OnceLock<String> = OnceLock::new();
static NICK_NAME: OnceLock<String> = OnceLock::new();
static AUTH_TOKEN: OnceLock<String> = OnceLock::new();

pub static PUBLIC_KEY_STR: &str = r#"-----BEGIN RSA PUBLIC KEY-----
MIIBCgKCAQEA4gRE9HOERcEUhKSNgqYtnVW9LIy+b5qM+jTEDoi956DhTytIAx+p
hOOrC/cI68+XXnPFZsNHy7ZxC2nONEzNYuS7ev9qxAxhhoRYLXDhsuqVsPg8KuJA
JRSDAhpdcw39vij4nccnlAjjFye73Qo7Mb7Gd7YAFBtX81I/u58QZrTqHVwkpqZN
WB+2YK//XV35PcOquKE4K9qyQfAydMKEmDjj6Q1yj6XoWJlaMspG/WRCUMM6G52J
P0Ln2gG3F5wFIy3jbYDq7AOVHkzX+ZHmlbCzceOVWBsBtfy8sjxOfFcdVGHOKgQE
P6fuBV/ohqqKDCwAGoA2RzIdkjtY6msWlwIDAQAB
-----END RSA PUBLIC KEY-----"#;

// 提供公共接口设置全局变量
pub fn set_host_ip(value: String) -> Result<(), Box<dyn Error>> {
    HOST_IP.set(value)?;
    Ok(())
}

// 提供公共接口获取全局变量
pub fn get_host_ip() -> &'static str {
    // AUTH_KEY.get().map(|s| s.as_str()).unwrap_or("")
    if let Some(key) = HOST_IP.get() {
        key.as_str()
    } else {
        error!("host ip not set, returning empty string.");
        ""
    }
}

// 提供公共接口设置全局变量
pub fn set_auth_key(value: String) -> Result<(), Box<dyn Error>> {
    AUTH_KEY.set(value)?;
    Ok(())
}

// 提供公共接口获取全局变量
pub fn get_auth_key() -> &'static str {
    // AUTH_KEY.get().map(|s| s.as_str()).unwrap_or("")
    if let Some(key) = AUTH_KEY.get() {
        key.as_str()
    } else {
        error!("Auth key not set, returning empty string.");
        ""
    }
}

// 提供公共接口设置全局变量
pub fn set_nick_name(value: String) -> Result<(), Box<dyn Error>> {
    NICK_NAME.set(value)?;
    Ok(())
}

// 提供公共接口获取全局变量
pub fn get_nick_name() -> &'static str {
    // AUTH_KEY.get().map(|s| s.as_str()).unwrap_or("")
    if let Some(key) = NICK_NAME.get() {
        key.as_str()
    } else {
        error!("nick name not set, returning empty string.");
        ""
    }
}

pub fn set_auth_token(value: String) -> Result<(), Box<dyn Error>> {
    AUTH_TOKEN.set(value)?;
    Ok(())
}

pub fn get_auth_token() -> &'static str {
    if let Some(key) = AUTH_TOKEN.get() {
        key.as_str()
    } else {
        error!("auth token not set, returning empty string.");
        ""
    }
}
