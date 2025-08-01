
use whoami::{platform, arch};
use serde::{Deserialize, Serialize};
use serde_json::{self};
use std::{collections::HashMap,env,fs,};
use std::path::Path;
use colored::Colorize;
use std::fs::File;
use std::io::Write;


// Custom crate
use autorok_utils::{
    request, download, extract, get_dirs
};



const IS_DEV_MODE: bool = cfg!(debug_assertions);

pub struct ReturnResult {
    pub status: bool,
    pub message: String
}
#[derive(Debug, Deserialize, Serialize)]
struct InstallManifest {
    status: bool,
    version: String
}

#[derive(Debug, Deserialize)]
struct Manifest(HashMap<String, HashMap<String, HashMap<String, String>>>);


pub async fn new() -> Result<ReturnResult, Box<dyn std::error::Error>> {
    
    let appdata_dir = get_dirs::appdata();

    if appdata_dir.is_empty() {
        return Ok(ReturnResult{status: false, message: "Failed to get appdata directory".to_string()})
    }

    
    let install_manifest_path = Path::new(&appdata_dir).join("install_manifest.json");
    
    if install_manifest_path.exists() {
        let json_str = fs::read_to_string(&install_manifest_path).expect("Failed to read file");
        match serde_json::from_str::<InstallManifest>(&json_str) {
            Ok(result) => {
                if result.status {
                    return Ok(ReturnResult{status: true, message: "Already installed.".to_string()})
                }
            },
            Err(e) => {
                eprintln!("Failed to parse install manifest JSON: {}", e);
            }
        }
    }
    

    let manifest: Manifest;
    if IS_DEV_MODE && env!("DEV_USE_LOCAL_MANIFEST") == "1" {
        let json_str = fs::read_to_string(env!("DEV_LOCAL_MANIFEST_PATH")).expect("Failed to read file");
        manifest = serde_json::from_str(&json_str).expect("Failed to parse JSON");
        
        
    }else{
        let body = request::get(env!("MANIFEST_URL")).await?;
        match serde_json::from_str::<Manifest>(&body) {
            Ok(result) => manifest = result,
            Err(e) => {
                eprintln!("Failed to parse manifest JSON: {}", e);
                return Ok(ReturnResult{status: false, message: "Failed to parse manifest JSON".to_string()});
            }
        };
    }

    let url = manifest.0[&platform().to_string()][&arch().to_string()]["url"].clone();
    let temp_dir = Path::new(&env::temp_dir()).join("autorok");
    match fs::create_dir_all(&temp_dir) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to create temp directory: {}", e);
            return Ok(ReturnResult{status: false, message: "Failed to create temp directory".to_string()});
        }
    };
    let output_file = temp_dir.join("zrok.tar.gz");
    
    // Download Zrok
    println!("{}{}", "=> 📦 Downloading zrok... | ".yellow(), url.yellow());
    let clone_output_file = output_file.to_path_buf();
    let result = tokio::task::spawn_blocking(move || {
        download::new(&url, clone_output_file.to_str().unwrap())
    }).await
    .expect("Blocking task panicked");

    if result.status  {
        println!("{}{}", "=> ✔ Downloaded zrok successfully ➡  ".green(), output_file.to_str().unwrap().green());
    }else{
        return Ok(ReturnResult{status: false, message: "Failed to download zrok".to_string()});
    }

    
    // Extract Zrok
    
    
    println!("{}{}", "=> 📦 Extracting zrok... | ".yellow(), &output_file.to_str().unwrap().yellow());
    let zrok_dir = Path::new(&appdata_dir).join("zrok");
    match fs::create_dir_all(&zrok_dir) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to create zrok directory: {}", e);
            return Ok(ReturnResult{status: false, message: "Failed to create zrok directory".to_string()});
        }
    };

    match extract::tar_gz(&output_file.to_str().unwrap(), &zrok_dir.to_str().unwrap()) {
        Ok(_) => {
            println!("{}{}", "=> ✔ Extracted zrok successfully ➡  ".green(), &zrok_dir.to_str().unwrap().green());
        },
        Err(e) => {
            eprintln!("Failed to extract zrok: {}", e);
            return Ok(ReturnResult{status: false, message: "Failed to extract zrok".to_string()});
        }
    }

    


    let result_install_manifest = InstallManifest {
        status: true,
        version: manifest.0[&platform().to_string()][&arch().to_string()]["version"].clone(),
    };

    
    let json_data = serde_json::to_string_pretty(&result_install_manifest).unwrap();

    
    let mut file = File::create(install_manifest_path)?;
    file.write_all(json_data.as_bytes())?;

    return Ok(ReturnResult{status: true, message: "Install successfully".to_string()})
}
