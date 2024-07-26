use std::{
    fmt::Display,
    io::{Stdout, Write},
    sync::{Arc, Mutex},
};

use crossterm::{cursor::MoveTo, execute, style::Print, terminal::Clear, QueueableCommand};
use state::{ButtonState, ControllerState};
use winapi::um::xinput::XINPUT_STATE;

pub mod state;
pub mod xinput;

impl state::ControllerState {
    /// # Safety
    ///
    /// last should be valid
    pub unsafe fn refresh(&mut self, last: XINPUT_STATE) -> XINPUT_STATE {
        let cur = xinput::refresh();
        let pad = cur.Gamepad;
        /* Button */
        let last_buttons = pad.wButtons;
        let cur_buttons = pad.wButtons;
        let clicked = (last_buttons ^ cur_buttons) & last_buttons;
        for i in 0..16 {
            if (clicked & (1 << i)) != 0 {
                self.buttons[i] = ButtonState::Clicked;
                continue;
            }
            if ((cur_buttons & (1 << i)) >> i) != 0 {
                self.buttons[i] = ButtonState::Pressed;
                continue;
            }
            self.buttons[i] = ButtonState::Original;
        }
        /* Trigger */
        self.triggers[state::LEFT] = (f64::from(pad.bLeftTrigger) / 2.55) as u8;
        self.triggers[state::RIGHT] = (f64::from(pad.bRightTrigger) / 2.55) as u8;
        /* Thumb */
        if pad.sThumbLX > 0 {
            self.thumbs[state::LEFT_X] = (f64::from(pad.sThumbLX) / 327.67) as i8;
        } else {
            self.thumbs[state::LEFT_X] = (f64::from(pad.sThumbLX) / 327.68) as i8;
        }
        if pad.sThumbLY > 0 {
            self.thumbs[state::LEFT_Y] = (f64::from(pad.sThumbLY) / 327.67) as i8;
        } else {
            self.thumbs[state::LEFT_Y] = (f64::from(pad.sThumbLY) / 327.68) as i8;
        }
        if pad.sThumbRX > 0 {
            self.thumbs[state::RIGHT_X] = (f64::from(pad.sThumbRX) / 327.67) as i8;
        } else {
            self.thumbs[state::RIGHT_X] = (f64::from(pad.sThumbRX) / 327.68) as i8;
        }
        if pad.sThumbRY > 0 {
            self.thumbs[state::RIGHT_Y] = (f64::from(pad.sThumbRY) / 327.67) as i8;
        } else {
            self.thumbs[state::RIGHT_Y] = (f64::from(pad.sThumbRY) / 327.68) as i8;
        }
        cur
    }
    pub fn buttons_display(
        &self,
        mut stdout: Stdout,
        pos_x: u16,
        pos_y: u16,
    ) -> Result<(), std::io::Error> {
        execute!(
            stdout,
            MoveTo(pos_x, pos_y),
            Clear(crossterm::terminal::ClearType::All),
            Print(self)
        )?;
        Ok(())
    }
    pub fn buttons_display_mt(
        &self,
        stdout: Arc<Mutex<Stdout>>,
        pos_x: u16,
        pos_y: u16,
    ) -> Result<(), std::io::Error> {
        let mut stdout = stdout.lock().unwrap();
        /*
        execute!(
            stdout,
            MoveTo(pos_x, pos_y),
            Clear(crossterm::terminal::ClearType::All),
            Print(self)
        )?;
        */
        stdout.queue(MoveTo(pos_x, pos_y))?;
        stdout.queue(Print(self))?;
        // stdout.queue(crossterm::style::Print(format!("{}", self)))?;
        stdout.flush()?;
        // IMPORTANT
        // Solution for dead lock
        drop(stdout);
        Ok(())
    }
}

impl Display for ButtonState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:>50}",
            match self {
                // Actually useless
                ButtonState::Clicked => "Clicked",
                ButtonState::Original => "Original",
                ButtonState::Pressed => "Pressed",
            }
        )
    }
}
impl Display for ControllerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, state) in self.buttons.iter().enumerate() {
            writeln!(f, "{} : {}", i, state)?;
        }
        writeln!(f, "")?;
        writeln!(f, "Left Trigger  : {:>30}%", self.triggers[0])?;
        writeln!(f, "Right Trigger : {:>30}%", self.triggers[1])?;
        writeln!(f, "Left Thumb X :  {:>10}%   Left Thumb Y : {:>10}% ", self.thumbs[0], self.thumbs[1])?;
        writeln!(f, "Right Thumb X : {:>10}%  Right Thumb Y : {:>10}% ", self.thumbs[2], self.thumbs[3])?;
        Ok(())
    }
}
/*
pub fn print_button_states(&self) {
    for (i, state) in self.button_states.iter().enumerate() {
        println!("Button {}: {:?}", i, state);
    }
}
*/

#[cfg(test)]
mod tests {}
