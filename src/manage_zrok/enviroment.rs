use autorok_utils::get_executor;
use std::io::{self, Write, BufReader, BufRead};
use colored::Colorize;
use std::process::{Command, Stdio};

pub fn enable() {
    println!("{}", "=== Enable Enviroment ===".purple());
    let zrok_path = get_executor::zrok();

    let mut input = String::new();

    print!("{}", "[>] Enter enviroment token: ".cyan());
    io::stdout().flush().unwrap(); // Ensures the prompt is shown immediately

    io::stdin().read_line(&mut input).unwrap();

    let zrok_token = input.trim();
    
    println!("{}", "⚙ Requesting zrok to enable...".yellow());

    let mut child = Command::new(zrok_path)
        .arg("enable")
        .arg(zrok_token)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn process");
    
    let mut last_line = String::new();

    if let Some(stdout) = &mut child.stdout {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            let unwarped_line = line.unwrap();
            if unwarped_line.trim().is_empty() {
                println!("{}",unwarped_line);
            }else{
                println!("=> {}", unwarped_line);
                last_line = unwarped_line;
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


    if last_line.trim().contains("zrok environment was successfully enabled") {
        println!("{}", "✅ Enviroment enabled successfully.".green());
    }

    println!("{}", "Press any key to continue...".yellow());
    let mut dummy = String::new();
    io::stdin().read_line(&mut dummy).unwrap();


}

pub fn disable() {
    println!("{}", "=== Disable Enviroment ===".purple());
    let zrok_path = get_executor::zrok();

    println!("{}", "⚙ Requesting zrok to disable...".yellow());

    let mut child = Command::new(zrok_path)
        .arg("disable")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn process");
    

    if let Some(stdout) = &mut child.stdout {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            let unwarped_line = line.unwrap();
            if unwarped_line.trim().is_empty() {
                println!("{}",unwarped_line);
            }else{
                println!("=> {}", unwarped_line);
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


    println!("{}", "Press any key to continue...".yellow());
    let mut dummy = String::new();
    io::stdin().read_line(&mut dummy).unwrap();

    
}

