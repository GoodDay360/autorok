use autorok_utils::get_executor;
use std::io::{self, Write, BufReader, BufRead};
use colored::Colorize;
use std::process::{Command, Stdio};

use crate::display_all_service;




pub fn new() {
    println!("{}", "=== Share Service ===".purple());
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
    match share_info.get(selected_id as usize) {
        Some(info) => {
            share_token = info.share_token.clone();
        }
        None => {
            println!("{}", "❌ Index out of bounds".red());
            return;
        }
    }

    let zrok_path = get_executor::zrok();

    println!("{}", "⚙ Requesting zrok to share service...".yellow());

    let mut child = Command::new(zrok_path)
        .arg("share")
        .arg("reserved")
        .arg(&share_token)
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