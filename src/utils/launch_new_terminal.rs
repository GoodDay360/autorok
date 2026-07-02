use launch_terminal::{open, is_installed, Terminal, Error};
use std::collections::HashMap;

/// Launches `command` in whatever terminal is available on this system.
/// Tries a preferred order per platform, falling back if not installed.
pub fn new(command: &str) -> Result<(), Error> {
    let candidates: &[Terminal] = if cfg!(target_os = "windows") {
        &[Terminal::WindowsDefault, Terminal::WSL]
    } else if cfg!(target_os = "macos") {
        &[
            Terminal::ITerm2,
            Terminal::WezTerm,
            Terminal::Warp,
            Terminal::Kitty,
            Terminal::Ghostty,
            Terminal::AppleTerminal, // last resort, always present on macOS
        ]
    } else {
        // Linux
        &[
            Terminal::GNOMETerminal,
            Terminal::Ptyxis,
            Terminal::Konsole,
            Terminal::Kitty,
            Terminal::Alacritty,
            Terminal::Ghostty,
            Terminal::Warp,
        ]
    };

    for terminal in candidates {
        match is_installed(terminal.clone()) {
            Ok(true) => return open(terminal.clone(), command, HashMap::new()),
            _ => continue, // not installed, or platform mismatch — try next
        }
    }

    Err(Error::NotSupported)
}