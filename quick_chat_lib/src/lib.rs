use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct QcMessage {
    pub nick_name: String, // 客户端用户的昵称
    pub message: String,   // 用户发的消息/或客户端发的指令内容
    pub cmd: String,       // 客户端发送的命令代码。代码定义请见readme的消息体命令定义
}

pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
