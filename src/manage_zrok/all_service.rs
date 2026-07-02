use crate::utils::{get_executor, short_hash};
use std::io::{self, Write, BufReader, BufRead};
use colored::Colorize;
use std::process::{Command, Stdio};
use serde_json::Value;
use tabled::{Tabled, Table, settings::Style};

#[derive(Tabled, Clone)]
pub struct ShareInfo {
    pub id: usize,
    pub backend_mode: String,
    pub backend_proxy_endpoint: String,
    pub share_mode: String,
    pub creator_id: String,
    pub share_token: String,

}

pub fn new() -> Vec<ShareInfo> {
    println!("{}", "⚙ Requesting info from zrok...".yellow());
    let zrok_path = get_executor::zrok();


    let mut child = Command::new(zrok_path)
        .arg("overview")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn process");
    
    let mut request_result = String::new();
    if let Some(stdout) = &mut child.stdout {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            let unwarped_line = line.unwrap();
            if !unwarped_line.trim().is_empty() {
                request_result.push_str(&unwarped_line);
            }
            
        }
    }


    if let Some(stderr) = &mut child.stderr {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            println!("{}{}", "=> ".red(), line.unwrap().red());
        }
    }

    let _ = child.wait(); 
    io::stdout().flush().unwrap();

    let mut share_info: Vec<ShareInfo> = Vec::new();
    

    match serde_json::from_str::<Value>(&request_result) {
        Ok(result) => {
            
            let mut index: usize = 0;
            if let Some(envs) = result.get("environments").and_then(|v| v.as_array()) {
                for env in envs {
                    let creator_id = short_hash::new(env.get("environment").and_then(|v| v.get("host")).unwrap_or(&Value::Null).as_str().unwrap_or(""), 10);
                    if let Some(shares) = env.get("shares").and_then(|s| s.as_array()) {
                        for share in shares {
                            share_info.push(ShareInfo {
                                id: index, 
                                backend_mode: share.get("backendMode").unwrap_or(&Value::Null).as_str().unwrap_or("").to_string(),
                                backend_proxy_endpoint: share.get("backendProxyEndpoint").unwrap_or(&Value::Null).as_str().unwrap_or("").to_string(),
                                share_mode: share.get("shareMode").unwrap_or(&Value::Null).as_str().unwrap_or("").to_string(),
                                creator_id: creator_id.clone(),
                                share_token: share.get("shareToken").unwrap_or(&Value::Null).as_str().unwrap_or("").to_string(),
                            });
                            index += 1;
                        }
                    }
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to parse request manifest JSON: {}", e);
        }
    }

    if share_info.len() == 0 {
        println!("{}", "❌ No services found.".red());
        return share_info;
    }


    let mut table = Table::new(&share_info);
    table.with(Style::rounded()); // Try Style::psql(), Style::modern() too
    println!("{}", table);

    return share_info;
    
}