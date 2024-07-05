use crate::{
    mouse::{click_mouse, get_mouse_position, move_mouse},
    types::MouseMove,
};

pub fn handle_click() {
    println!("handle_click");
    click_mouse("1");
}
pub fn handle_keypress() {}

pub fn handle_move(cursor: MouseMove) {
    let cursor_sensibility = 5;

    let (mut x, mut y) = get_mouse_position();

    if cursor.dx < 0 {
        x = x - cursor_sensibility;
    } else {
        x = x + cursor_sensibility;
    }

    if cursor.dy < 0 {
        y = y - cursor_sensibility;
    } else {
        y = y + cursor_sensibility;
    }

    move_mouse(x, y);
}
