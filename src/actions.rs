use crate::{
    mouse::{click_mouse, get_mouse_position, move_mouse},
    types::Drag,
};

pub fn handle_btn1() {
    println!("handle_btn1");
}

pub fn handle_touch() {
    click_mouse("1");
}

pub fn handle_drag(drag: Drag) {
    let drag_sensibility = 5;

    let (mut x, mut y) = get_mouse_position();

    if drag.dx < 0 {
        x = x - drag_sensibility;
    } else {
        x = x + drag_sensibility;
    }

    if drag.dy < 0 {
        y = y - drag_sensibility;
    } else {
        y = y + drag_sensibility;
    }

    move_mouse(x, y);
}
