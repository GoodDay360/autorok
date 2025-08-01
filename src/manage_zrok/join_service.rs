use autorok_utils::get_executor;
use std::io::{self, Write, BufReader, BufRead};
use colored::Colorize;
use std::process::{Command, Stdio};

use crate::display_all_service;


pub fn new() {
    println!("{}", "=== Join Service ===".purple());
    let share_info = display_all_service::new();
    if share_info.len() == 0 {
        println!("{}", "Press any key to continue...".yellow());
        let mut dummy = String::new();
        io::stdin().read_line(&mut dummy).unwrap();
        return;
    }

    let mut input = String::new();
    let mut selected_id: u32;

    loop {
        print!("{}", "[>] Enter service id [-1 to exit]: ".cyan());
        io::stdout().flush().unwrap(); // Show the prompt immediately

        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<i32>() {
            Ok(id) => {
                if id < 0 {return}
                selected_id = id as u32;
                if (selected_id as usize) > share_info.len() {
                    println!("{}", "❌ Invalid input. Please enter a valid id.".red());
                }else{
                    break;
                }
                
            }
            Err(_) => {
                println!("{}", "❌ Invalid input. Please enter a number only.".red());
            }
        }
    }

    let share_token:String;
    let backend_ip: String;
    let backend_port: String;

    match share_info.get(selected_id as usize) {
        Some(info) => {
            share_token = info.share_token.clone();
            let backend_parts: Vec<&str> = info.backend_proxy_endpoint.split(':').collect();
            backend_ip = backend_parts[0].to_string();
            backend_port = backend_parts[1].to_string();
        }
        None => {
            println!("{}", "❌ Index out of bounds".red());
            return;
        }
    }

    // Request input for ip address
    let mut ip_addr: String = backend_ip.clone();
    let mut input = String::new();
    print!("{}", "[>] Enter bind ip address [Default: same as host]: ".cyan().to_string());
    io::stdout().flush().unwrap(); // Show the prompt immediately
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    if !input.trim().is_empty() {
        ip_addr = input.trim().to_string();
    }
    println!("{}{}", "=> Bind IP Address: ".yellow(), ip_addr.yellow());
    

    // Request input for port
    let mut port: String = backend_port.clone();
    let mut input = String::new();
    print!("{}", "[>] Enter bind port [Default: same as host]: ".cyan().to_string());
    io::stdout().flush().unwrap(); // Show the prompt immediately
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    if !input.trim().is_empty() {
        port = input.trim().to_string();
    }
    println!("{}{}", "=> Bind Port: ".yellow(), port.yellow());


    let zrok_path = get_executor::zrok();

    println!("{}", "⚙ Requesting zrok to join service...".yellow());

    let mut child = Command::new(zrok_path)
        .arg("access")
        .arg("private")
        .arg(&share_token)
        .arg("--bind").arg(ip_addr+&":".to_string()+&port)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn process");
    

    if let Some(stdout) = &mut child.stdout {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            let unwarped_line = line.unwrap();
            println!("{}",unwarped_line);
        }
    }

    if let Some(stderr) = &mut child.stderr {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            println!("{}{}", "=> ", line.unwrap());
        }
    }

    let _ = child.wait(); 
    io::stdout().flush().unwrap();


    println!("{}", "Press any key to continue...".yellow());
    let mut dummy = String::new();
    io::stdin().read_line(&mut dummy).unwrap();

    
    
}