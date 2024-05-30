use std::process::Command;
use std::str;

pub fn get_mouse_position() -> (i32, i32) {
    let output = Command::new("xdotool")
        .arg("getmouselocation")
        .output()
        .expect("Failed to execute xdotool command");

    let output_str = str::from_utf8(&output.stdout).expect("Invalid UTF-8 sequence");
    let parts: Vec<&str> = output_str.split_whitespace().collect();

    let mut x = 0;
    let mut y = 0;

    for part in parts {
        let key_value: Vec<&str> = part.split(':').collect();
        if key_value.len() == 2 {
            match key_value[0] {
                "x" => x = key_value[1].parse().unwrap_or(0),
                "y" => y = key_value[1].parse().unwrap_or(0),
                _ => {}
            }
        }
    }

    (x, y)
}

pub fn move_mouse(x: i32, y: i32) {
    Command::new("xdotool")
        .arg("mousemove")
        .arg(x.to_string())
        .arg(y.to_string())
        .output()
        .expect("Failed to execute xdotool command");
}
