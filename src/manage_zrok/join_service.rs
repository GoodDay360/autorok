use std::io::{self, Write, BufReader, BufRead};
use colored::Colorize;
use std::process::{Command, Stdio};

use crate::all_service;
use crate::utils::{get_executor, launch_new_terminal};

use crate::current_share_join_service::CURRENT_JOIN_SERVICE;

pub fn new() -> anyhow::Result<()> {
    println!("{}", "=== Join Service ===".purple());
    let share_info = all_service::new();
    if share_info.len() == 0 {
        println!("{}", "Press any key to continue...".yellow());
        io::stdin().read_line(&mut String::new()).unwrap();
        return Ok(());
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
                if id < 0 {return Ok(());}
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
    let share_mode: String;

    match share_info.get(selected_id as usize) {
        Some(info) => {
            share_token = info.share_token.clone();
            let backend_parts: Vec<&str> = info.backend_proxy_endpoint.split(':').collect();
            backend_ip = backend_parts[0].to_string();
            backend_port = backend_parts[1].to_string();
            share_mode = info.share_mode.clone();
        }
        None => {
            println!("{}", "❌ Index out of bounds".red());
            return Ok(());
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

    
    let command = format!(
        "{} access {} {} --bind {}:{}",
        zrok_path, share_mode, share_token, ip_addr, port
    );


    launch_new_terminal::new(&command)
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;


    
    println!("{}", "=> Spawned zrok network. Check 'Current Share/Join Service' command for more information.".yellow());
    println!("{}", "Press any key to continue...".yellow());
    let mut dummy = String::new();
    io::stdin().read_line(&mut dummy).unwrap();

    
    Ok(())
}