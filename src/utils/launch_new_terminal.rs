use launch_terminal::{is_installed, open, Error, Terminal};
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

    #[cfg(target_os = "linux")]
    {
        return launch_linux(command, candidates);
    }

    #[cfg(not(target_os = "linux"))]
    for terminal in candidates {
        match is_installed(terminal.clone()) {
            Ok(true) => return open(terminal.clone(), command, HashMap::new()),
            _ => continue, // not installed, or platform mismatch — try next
        }
    }

    #[cfg(not(target_os = "linux"))]
    Err(Error::NotSupported)
}

#[cfg(target_os = "linux")]
fn launch_linux(command: &str, candidates: &[Terminal]) -> Result<(), Error> {
    for terminal in candidates {
        if matches!(is_installed(terminal.clone()), Ok(true)) {
            if let Err(error) = open(terminal.clone(), command, HashMap::new()) {
                eprintln!("Failed to launch {}: {error}", terminal_name(terminal));
            } else {
                return Ok(());
            }
        }
    }

    let session_name = format!(
        "autorok-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    );
    let screen_result = std::process::Command::new("screen")
        .args(["-dmS", &session_name, "sh", "-c", command])
        .status();

    match screen_result {
        Ok(status) if status.success() => {
            eprintln!(
                "No terminal could be launched. The command is running in screen; use \
                 `screen -r {session_name}` to view your share service."
            );
            Ok(())
        }
        Ok(status) => Err(Error::IOError(std::io::Error::other(format!(
            "screen failed to start the command (exit status: {status})"
        )))),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            eprintln!(
                "No terminal could be launched and `screen` is not installed. \
                 Install screen (for example, `sudo apt install screen`) and try again."
            );
            Err(Error::IOError(error))
        }
        Err(error) => Err(Error::IOError(error)),
    }
}

#[cfg(target_os = "linux")]
fn terminal_name(terminal: &Terminal) -> &'static str {
    match terminal {
        Terminal::GNOMETerminal => "GNOME Terminal",
        Terminal::Ptyxis => "Ptyxis",
        Terminal::Konsole => "Konsole",
        Terminal::Kitty => "Kitty",
        Terminal::Alacritty => "Alacritty",
        Terminal::Ghostty => "Ghostty",
        Terminal::Warp => "Warp",
        _ => "terminal",
    }
}
