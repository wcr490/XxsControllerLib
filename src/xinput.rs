use std::mem::zeroed;

use winapi::{
    shared::minwindef::DWORD,
    um::xinput::{XInputGetState, XINPUT_STATE},
};

const DEFAULT_USER_INDEX: DWORD = 0;
pub fn refresh() -> XINPUT_STATE {
    unsafe {
        let mut n = zeroed();
        XInputGetState(DEFAULT_USER_INDEX, &mut n);
        n
    }
}
