use crate::get_dirs;
use std::{path::Path};
use whoami::{platform, Platform};


pub fn zrok() -> String {
    let appdata_dir = get_dirs::appdata();
    match platform() {
        Platform::Windows => return Path::new(&appdata_dir).join("zrok").join("zrok.exe").display().to_string(),
        _ => {
            eprintln!("Unsupported Platform");
            return "".to_string();
        },
    }
}

