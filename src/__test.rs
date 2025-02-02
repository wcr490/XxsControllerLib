use std::io::stdout;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType;
use winapi::um::xinput::XINPUT_STATE;

#[allow(unused_imports)]
use xci::state;
use xci::state::ButtonState;
#[allow(unused_imports)]
use xci::xinput;

pub fn main() -> Result<(), std::io::Error> {
    let mut controller = state::ControllerState::new();
    let _con = Arc::new(Mutex::new(controller));
    let _clone_con = Arc::clone(&_con);
    let stdout = Arc::new(Mutex::new(stdout()));

    let display_thread = {
        let stdout = Arc::clone(&stdout);
        let _con = Arc::clone(&_con);
        {
            let mut stdout = stdout.lock().unwrap();
            execute!(stdout, Clear(ClearType::All))?;
        }
        thread::spawn(move || loop {
            {
                let con = _con.lock().unwrap();
                let stdout = Arc::clone(&stdout);
                let _ = con.buttons_display_mt(stdout, 0, 0);
            }
            thread::sleep(Duration::from_millis(30));
        })
    };

    let update_thread = {
        let _clone_con = Arc::clone(&_clone_con);
        thread::spawn(move || unsafe {
            let mut _state: XINPUT_STATE = std::mem::zeroed();
            loop {
                let mut con = _clone_con.lock().unwrap();
                _state = con.refresh(_state);
            }
        })
    };
    display_thread.join().unwrap();
    update_thread.join().unwrap();
    /*
    unsafe {
        let mut stdout = stdout();
        let mut _state: XINPUT_STATE = std::mem::zeroed();
        let _ = execute!(stdout, Clear(ClearType::All));
        loop {
            _state = controller.refresh(_state);
            let _ = execute!(stdout, MoveTo(0, 10));
            println!("{:<20}", controller.trigger[0]);
            println!("{:<20}", controller.trigger[1]);
        }
    }
    */

    Ok(())
}
