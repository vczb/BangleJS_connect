use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Click {}
#[derive(Serialize, Deserialize)]
pub struct MouseMove {
    pub x: i32,
    pub y: i32,
    pub b: i32,
    pub dx: i32,
    pub dy: i32,
}
#[derive(Serialize, Deserialize)]
pub struct KeyPress {
    pub keycode: Vec<i32>,
}
// TODO
// #[derive(Serialize, Deserialize)]
// pub struct Mic {}
// #[derive(Serialize, Deserialize)]
// pub struct Cam {}
// #[derive(Serialize, Deserialize)]
// pub struct Vol {}

#[derive(Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum Events {
    #[serde(rename = "click")]
    Click(Click),
    #[serde(rename = "mouse_move")]
    MouseMove(MouseMove),
    #[serde(rename = "keypress")]
    KeyPress(KeyPress),
}
