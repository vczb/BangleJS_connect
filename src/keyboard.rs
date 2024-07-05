use std::process::Command;
use std::str;

pub fn keypress(key: &str) {
    Command::new("xdotool")
        .arg("key")
        .arg(key)
        .output()
        .expect("Failed to execute xdotool command");
}
