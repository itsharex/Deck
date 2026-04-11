use crate::output::OutputMode;
use serde_json::json;

const LOGO: &str = include_str!("../logo.ans");

fn terminal_width() -> Option<u16> {
    unsafe {
        let mut ws: libc::winsize = std::mem::zeroed();
        if libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &mut ws) == 0 && ws.ws_col > 0 {
            Some(ws.ws_col)
        } else {
            None
        }
    }
}

pub fn run(output: OutputMode) {
    let version = env!("CARGO_PKG_VERSION");
    match output {
        OutputMode::Text => {
            if terminal_width().unwrap_or(80) >= 50 {
                print!("{}", LOGO);
            }
            println!("deckclip {}", version);
            println!("\x1b]8;;https://deckclip.app\x07DeckClip@Deck\x1b]8;;\x07");
            println!("© 2024-2026 Yuze Pan. All rights reserved.");
        }
        OutputMode::Json => {
            println!("{}", json!({ "version": version }));
        }
    }
}
