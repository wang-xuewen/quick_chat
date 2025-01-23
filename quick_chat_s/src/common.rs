//! 本模块存放所有全局对象定义以及各种常量、项目级别的通用函数等.
//!
use std::sync::OnceLock;
// 定义全局变量
static AUTH_KEY: OnceLock<String> = OnceLock::new();

// 提供公共接口设置全局变量
pub fn set_auth_key(value: String) {
    AUTH_KEY.set(value).expect("Failed to set auth key");
}

// 提供公共接口获取全局变量
pub fn get_auth_key() -> &'static String {
    AUTH_KEY.get().expect("auth key not set")
}
