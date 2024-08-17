extern crate winapi;

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

pub const LEFT: usize = 0;
pub const RIGHT: usize = 1;

pub const LEFT_X: usize = 0;
pub const LEFT_Y: usize = 1;
pub const RIGHT_X: usize = 2;
pub const RIGHT_Y: usize = 3;

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct ControllerState {
    pub is_connected: bool,
    pub buttons: [ButtonState; BUTTON_NUM],
    pub triggers: [u8; 2],
    pub thumbs: [i8; 4],
}

#[allow(dead_code)]
#[derive(Debug, Default, PartialEq, Eq)]
pub enum ButtonState {
    #[default]
    Original,
    Pressed,
    /// Pressed + Released = clicked
    Clicked,
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
        if let 0..=15 = idx {
            self.buttons[idx] = state
        }
    }
}
