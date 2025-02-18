//! 本模块存放所有全局对象定义以及各种常量、项目级别的通用函数等.
//!
use log::error;
use std::error::Error;
use std::sync::OnceLock;
// 定义全局变量
static AUTH_KEY: OnceLock<String> = OnceLock::new();
pub static PRIVATE_KEY_STR: &str = r#"-----BEGIN RSA PRIVATE KEY-----
MIIEpQIBAAKCAQEA4gRE9HOERcEUhKSNgqYtnVW9LIy+b5qM+jTEDoi956DhTytI
Ax+phOOrC/cI68+XXnPFZsNHy7ZxC2nONEzNYuS7ev9qxAxhhoRYLXDhsuqVsPg8
KuJAJRSDAhpdcw39vij4nccnlAjjFye73Qo7Mb7Gd7YAFBtX81I/u58QZrTqHVwk
pqZNWB+2YK//XV35PcOquKE4K9qyQfAydMKEmDjj6Q1yj6XoWJlaMspG/WRCUMM6
G52JP0Ln2gG3F5wFIy3jbYDq7AOVHkzX+ZHmlbCzceOVWBsBtfy8sjxOfFcdVGHO
KgQEP6fuBV/ohqqKDCwAGoA2RzIdkjtY6msWlwIDAQABAoIBAQCTDjYTW0nX3N3N
xAcy147cTNnuL5EEqmlJrqVV05DEMcAHM+EB70rr5Pn4qf4K+CHD2c25pBuFpyn8
RBZfnw0PaEW3rJI6Btg5nsiEoGqpb8p/v7PeLyGPPr5S0va0kojjfmLsvZS50+J1
wWuRUUPchQU7BJcefgAGmCBRohgYyipAAMiTH2U8z00LyFI9QU5xGybP+GOoPn59
P7QnEa4/Tn/+6bY2jcanI3JzznZL1gvvxx+lSurZIapl+P91TXiXQ/AJEJ7TPTDc
Riv83iBevzlxKyTSo8oGjLKK6Ml/Sh0Zxasf+812G+E9FFIBkkhInQJX0FwMlvHZ
YoSiWM1JAoGBAOegHN2Rcp3xP/S4KR5Pd7Knjv3pZ+4awF3/ffUGdIzK3dHGKQy3
YvT8By/QrhjeArpWrmmTlWup7FnSFzFfUzyxWjgXSIQDTIRijvDq0ZtlKU8UHYPN
7ePQl412QPP+LtCt6+Yd8AuvHCnfyisSYy35anwtr7AkelEtz8R7hqFbAoGBAPnN
DxnphprZqnaZxI3YxrqDlcKKtA81qG4i/HVtjJJdf3sedQ9imlHoMLq5/1mnKaB8
E7/YR1Ib0OAN73LfapDh/sFhzaiPM37g+2VflUU5BToDtDnQMt0/RV7t6jd8O4tu
QZLVgXApwY507mmyz4W+taiQ7+M9bAxXO+3VcYh1AoGBAI7WZ1af3l3WK4mflAPU
H821lPGyYVwtdRnCeAuFWpSEejxmBmSIJudLEKeE+gftySLeV5pV39xQIqfVbmYN
Egiomili+l4mpqYxHVMmi/JXdR0GG5lvgdduiDc9iJqu0nHv/zyek6yw5R5Rmpvr
L+xnFirUBbcLF78+EBVr079nAoGAD86E/RvE07mgSr7yLBOih5zZ9iR2vluj28xE
811KPtzBu1WzDJUttK8fnkE0wkSMosYXLdWOtchi0DqxgzBV+vMB/tSkgd0F4ip0
XfbNaELybLhdSCc/gLaHOjmNz5MB5ZHFfngaJ7HMuKn3iCKzdQAbWJ5LP7LcSm+e
sC8Ibx0CgYEAg2AGQd2FFvekl4LU+vho5nmJ+ieDeWMzEW9kY5Gv3UfvSkJCXgNL
seTQ1kWIIiQE6Yc9xT/FSs3YWC9YuUK5DMog0bH+xnqFxc1vVqMtR+8Khf5BhkVC
eY7i0K6c9dKEiAWBsvd3C8/ktcXSps8wjxGVH+X/2Re316biQfk6QV8=
-----END RSA PRIVATE KEY-----"#;

// pub static PUBLIC_KEY_STR: &str = r#"-----BEGIN RSA PUBLIC KEY-----
// MIIBCgKCAQEA4gRE9HOERcEUhKSNgqYtnVW9LIy+b5qM+jTEDoi956DhTytIAx+p
// hOOrC/cI68+XXnPFZsNHy7ZxC2nONEzNYuS7ev9qxAxhhoRYLXDhsuqVsPg8KuJA
// JRSDAhpdcw39vij4nccnlAjjFye73Qo7Mb7Gd7YAFBtX81I/u58QZrTqHVwkpqZN
// WB+2YK//XV35PcOquKE4K9qyQfAydMKEmDjj6Q1yj6XoWJlaMspG/WRCUMM6G52J
// P0Ln2gG3F5wFIy3jbYDq7AOVHkzX+ZHmlbCzceOVWBsBtfy8sjxOfFcdVGHOKgQE
// P6fuBV/ohqqKDCwAGoA2RzIdkjtY6msWlwIDAQAB
// -----END RSA PUBLIC KEY-----"#;

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
// pub fn get_auth_key() -> &'static String {
//     AUTH_KEY.get().expect("auth key not set")
// }
// pub fn get_auth_key() -> Result<&'static String, Box<dyn Error>> {
//     // Option 类型的 ok_or_else 方法可以将 Option<T> 转换为 Result<T, E>
//     // into() 是 Rust 中的一种通用转换方法，可以将字符串转换为实现了 Error trait 的类型。
//     AUTH_KEY
//         .get()
//         .ok_or_else(|| "Failed to get auth key".into())
// }
