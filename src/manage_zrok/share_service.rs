use crate::{manage_zrok::current_share_join_service::CURRENT_SHARE_SERVICE, utils::{get_executor, launch_new_terminal}};
use std::io::{self, Write, BufReader, BufRead};
use colored::Colorize;
use std::process::{Command, Stdio};

use crate::all_service;




pub fn new() -> anyhow::Result<()> {
    println!("{}", "=== Share Service ===".purple());
    let share_info = all_service::new();
    if share_info.len() == 0 {
        println!("{}", "Press any key to continue...".yellow());
        let mut dummy = String::new();
        io::stdin().read_line(&mut dummy).unwrap();
        return Ok(());
    }

    println!("{}", "[Note] Sharing only allow for same environment machine that created it only.".yellow());

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
    match share_info.get(selected_id as usize) {
        Some(info) => {
            share_token = info.share_token.clone();
        }
        None => {
            println!("{}", "❌ Index out of bounds".red());
            return Ok(());
        }
    }

    let zrok_path = get_executor::zrok();

    println!("{}", "⚙ Requesting zrok to share service...".yellow());

    
    let command = format!("{} share reserved {}", zrok_path, share_token);

    launch_new_terminal::new(&command)
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;


    println!("{}", "=> Spawned zrok network. Check 'Current Share/Join Service' command for more information.".yellow());
    println!("{}", "Press any key to continue...".yellow());
    let mut dummy = String::new();
    io::stdin().read_line(&mut dummy).unwrap();

    
    Ok(())
}