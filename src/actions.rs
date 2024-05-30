use crate::{
    mouse::{get_mouse_position, move_mouse},
    types::Drag,
};

pub fn handle_btn1() {}

pub fn handle_drag(drag: Drag) {
    let mouse_sensibility = 5;

    let (mut x, mut y) = get_mouse_position();

    if (drag.dx < 0) {
        x = x - mouse_sensibility;
    } else {
        x = x + mouse_sensibility;
    }

    if (drag.dy < 0) {
        y = y - mouse_sensibility;
    } else {
        y = y + mouse_sensibility;
    }

    move_mouse(x, y);
}
