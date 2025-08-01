use autorok_utils::get_executor;
use std::io::{self, Write, BufReader, BufRead};
use colored::Colorize;
use std::process::{Command, Stdio};
use dialoguer::{Select, theme::ColorfulTheme};




pub fn new() {
    println!("{}", "=== Create Service ===".purple());

    // Request input for ip address
    let mut ip_addr: String = "127.0.0.1".to_string();
    let mut input = String::new();
    print!("{}", "[>] Enter ip address [Default: 127.0.0.1]: ".cyan().to_string());
    io::stdout().flush().unwrap(); // Show the prompt immediately
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    if !input.trim().is_empty() {
        ip_addr = input.trim().to_string();
    }
    println!("{}{}", "=> IP Address: ".yellow(), ip_addr.yellow());
    

    // Request input for port
    let mut port: String = "25565".to_string();
    let mut input = String::new();
    print!("{}", "[>] Enter port [Default: 25565]: ".cyan().to_string());
    io::stdout().flush().unwrap(); // Show the prompt immediately
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    if !input.trim().is_empty() {
        port = input.trim().to_string();
    }
    println!("{}{}", "=> Port: ".yellow(), port.yellow());


    // Select Backend Mode
    let mut backend_mode: String = "tcpTunnel".to_string();
    println!("{}", "[>] Select Backend Mode: ".cyan().to_string());
    let items = vec![
        "tcpTunnel -> Require for Minecraft Java Edition or Bedrock Edition", 
        "udpTunnel -> Require for Minecraft Bedrock Edition", 
    ];
    let theme = ColorfulTheme::default();
    let selected = Select::with_theme(&theme)
        .items(&items)
        .default(0)
        .interact()
        .unwrap();

    match selected {
        0 => {
            backend_mode = "tcpTunnel".to_string();
        }
        1 => {
            backend_mode = "udpTunnel".to_string();
        }
        _ => {
            println!("{}","Invalid option!".yellow());
        }
    }

    println!("{}{}", "=> Backend Mode: ".yellow(), backend_mode.yellow());

    // Request input for unique name
    let mut unique_name: String = "".to_string();
    let mut input = String::new();
    print!("{}", "[>] Enter unique name [Default: random]: ".cyan().to_string());
    io::stdout().flush().unwrap(); // Show the prompt immediately
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    if !input.trim().is_empty() {
        unique_name = input.trim().to_string();
    }

    println!("{}{}", "=> Unique Name: ".yellow(),  if unique_name.is_empty() { "random".yellow() } else { unique_name.yellow() });

    let zrok_path = get_executor::zrok();

    println!("{}", "⚙ Requesting zrok to create service...".yellow());

    let mut child = Command::new(zrok_path)
        .arg("reserve")
        .arg("private")
        .arg(ip_addr+&":".to_string()+&port)
        .arg("--backend-mode").arg(backend_mode)
        .arg("--unique-name").arg(unique_name)
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