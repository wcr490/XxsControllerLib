use state::ButtonState;
use winapi::um::xinput::XINPUT_STATE;

pub mod state;
pub mod xinput;

impl state::ControllerState {
    pub unsafe fn refresh(&mut self, last: XINPUT_STATE) -> XINPUT_STATE {
        let cur = xinput::refresh();
        let last_buttons = last.Gamepad.wButtons;
        let cur_buttons = cur.Gamepad.wButtons;
        let clicked = (last_buttons ^ cur_buttons) & last_buttons;
        for i in 0..16 {
            /*
            let clicked_flag = if clicked & (1 << i) != 0 { 2 } else { 0 };
            let pressed_flag = cur_buttons & (1 << i) >> i;
            */
            let _flag: u8 =
                if clicked & (1 << i) != 0 { 2 } else { 0 } + ((cur_buttons & (1 << i)) >> i) as u8;
            self.set_with_index(i, ButtonState::from_u8(_flag));
        }
        cur
    }
}
#[cfg(test)]
mod tests {}
