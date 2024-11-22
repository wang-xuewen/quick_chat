use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct QcMessage {
    pub nick_name: String,
    pub message: String,
}

pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
