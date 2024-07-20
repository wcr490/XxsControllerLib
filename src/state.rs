extern crate winapi;

use std::default;

const BUTTON_NUM: usize = 16;
pub const DPAD_UP: usize = 0;
pub const DPAD_DOWN: usize = 1;
pub const DPAD_LEFT: usize = 2;
pub const DPAD_RIGHT: usize = 3;
pub const MENU: usize = 4;
pub const SELECT: usize = 5;
pub const LEFT_STICK: usize = 6;
pub const RIGHT_STICK: usize = 7;
pub const LEFT_BUMPER: usize = 8;
pub const RIGHT_BUMPER: usize = 9;
pub const ACTION_A: usize = 12;
pub const ACTION_B: usize = 13;
pub const ACTION_X: usize = 14;
pub const ACTION_Y: usize = 15;

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct ControllerState {
    is_connected: bool,
    buttons: [ButtonState; BUTTON_NUM],
    /*    dpad_up: ButtonState,
    dpad_down: ButtonState,
    dpad_left: ButtonState,
    dpad_right: ButtonState,
    menu: ButtonState,
    select: ButtonState,
    left_stick: ButtonState,
    right_stick: ButtonState,
    left_bumper: ButtonState,
    right_bumper: ButtonState,
    action_a: ButtonState,
    action_b: ButtonState,
    action_x: ButtonState,
    action_y: ButtonState,*/
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum ButtonState {
    Original,
    Pressed,
    /// Pressed + Released = clicked
    Clicked,
}
impl Default for ButtonState {
    fn default() -> Self {
        ButtonState::Original
    }
}
impl ButtonState {
    pub fn from_u8(v: u8) -> Self {
        match v {
            1 => Self::Pressed,
            2 => Self::Clicked,
            _ => Self::Original,
        }
    }
}
impl ControllerState {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_with_index(&mut self, idx: usize, state: ButtonState) {
        match idx {
            0..=15 => self.buttons[idx] = state,
            _ => {}
        };
    }
}
