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
        let last_buttons = last.Gamepad.wButtons;
        let cur_buttons = cur.Gamepad.wButtons;
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
            "{}",
            match self {
                ButtonState::Clicked => "C",
                ButtonState::Original => "O",
                ButtonState::Pressed => "P",
            }
        )
    }
}
impl Display for ControllerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, state) in self.buttons.iter().enumerate() {
            writeln!(f, "{} is {}", i, state)?;
        }
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
