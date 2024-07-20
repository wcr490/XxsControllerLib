use std::mem::zeroed;

use winapi::{
    shared::minwindef::DWORD,
    um::{
        self,
        xinput::{XInputGetState, XINPUT_STATE},
    },
};

const DEFAULT_USER_INDEX: DWORD = 0;
pub unsafe fn refresh() -> XINPUT_STATE {
    let mut n = zeroed();
    XInputGetState(DEFAULT_USER_INDEX, &mut n);
    n
}
