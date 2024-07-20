use std::mem::zeroed;

use winapi::um::xinput::XINPUT_STATE;

use xci::state;
use xci::xinput;
pub fn main() {
    let mut controller = state::ControllerState::new();
    unsafe {
        let mut _state: XINPUT_STATE = zeroed();
        loop {
            _state = controller.refresh(_state);
            println!("{}", _state.Gamepad.wButtons);
        }
    }
}
