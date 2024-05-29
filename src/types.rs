use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Drag {
    pub x: i32,
    pub y: i32,
    pub b: i32,
    pub dx: i32,
    pub dy: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Btn1 {
    command: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event")]
pub enum Command {
    #[serde(rename = "drag")]
    Drag(Drag),
    #[serde(rename = "btn1")]
    Btn1(Btn1),
}
