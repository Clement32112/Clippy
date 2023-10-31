use crossterm::{
    event::{self, KeyCode, KeyEvent},
    terminal::{self, ClearType},
};
fn detect() {
    terminal::enable_raw_mode().expect("failed to enable mode");
    loop {
        if event::poll(std::time::Duration::from_millis(100)).expect("Polling for events failed") {
            if let event::Event::Key(key_event) =
                event::read().expect("Reading from keyboardinput failed")
            {
                match key_event {
                    KeyEvent {
                        code,
                        state,
                        kind,
                        modifiers,
                    } => match code {
                        KeyCode::Esc => {
                            break;
                        }
                        _ => {
                            println!("{:?}", code)
                        }
                    },
                }
            }
        }
    }
    terminal::disable_raw_mode().expect("Failed to disable raw mode");
    println!("{}", terminal::Clear(ClearType::All));
}

pub fn hello() {
    println!("Hello from key bind");
}
