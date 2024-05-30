use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Drag {
    pub x: i32,
    pub y: i32,
    pub b: i32,
    pub dx: i32,
    pub dy: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Touch {}

#[derive(Serialize, Deserialize)]
pub struct Btn1 {}

#[derive(Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum Events {
    #[serde(rename = "touch")]
    Touch(Touch),
    #[serde(rename = "drag")]
    Drag(Drag),
    #[serde(rename = "btn1")]
    Btn1(Btn1),
}
