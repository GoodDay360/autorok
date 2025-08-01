

use colored::Colorize;
use std::path::Path;
use std::fs;
use std::io::ErrorKind;
use clearscreen;

use autorok_utils::get_dirs;
use crate::manage_zrok::{
    check_install
};


pub async fn new() -> bool{
    let zrok_dir = Path::new(&get_dirs::appdata()).join("zrok");
    match fs::remove_dir_all(&zrok_dir) {
        Ok(_) => println!("Deleted: {}", zrok_dir.display()),
        Err(e) if e.kind() == ErrorKind::NotFound => {
            println!("=> Path not found, ignoring: {}", zrok_dir.display());
        }
        Err(e) => {
            eprintln!("=> Failed to delete {}: {}", zrok_dir.display(), e);
            return false;
        },
    }

    let install_manifest_path = Path::new(&get_dirs::appdata()).join("install_manifest.json");
    match fs::remove_file(&install_manifest_path) {
        Ok(_) => println!("Deleted: {}", install_manifest_path.display()),
        Err(e) if e.kind() == ErrorKind::NotFound => {
            println!("=> Path not found, ignoring: {}", install_manifest_path.display());
        }
        Err(e) => {
            eprintln!("=> Failed to delete {}: {}", install_manifest_path.display(), e);
            return false;
        },
    }

    clearscreen::clear().unwrap();
    let check_install_result:bool;
    match check_install::new().await {
        Ok(result) => {
            check_install_result = result.status;
            if result.status {
                println!("{}{}", "✅ Checking Result: ".green(), result.message.green());
            } else {
                println!("{}{}", "⚠️ Failed with message: ".red(), result.message.red());
            }
        }
        Err(e) => {
            println!("❌ Unexpected error: {}", e);
            check_install_result = false;
        }
    }
    return check_install_result;
}