use std::{path::Path, fs};
use directories::BaseDirs;


pub fn appdata() -> String {
    let appdata_dir: String;
    if let Some(base_dirs) = BaseDirs::new() {
        let base_dir_clone = base_dirs.data_dir().display().to_string();
        appdata_dir = Path::new(&base_dir_clone).join("autorok").display().to_string();
        match fs::create_dir_all(&appdata_dir) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Failed to create zrok directory: {}", e);
                return  "".to_string();
            }
        };
        return appdata_dir;
    } else {
        eprintln!("Could not determine AppData directory.");
        return "".to_string();
    }
    
}




