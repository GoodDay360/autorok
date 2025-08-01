
use colored::Colorize;
use dialoguer::{Select, theme::ColorfulTheme};
use clearscreen;
use std::io::{self};

mod manage_zrok;
use manage_zrok::{
    check_install, enviroment, display_all_service, share_service, delete_service, create_service, join_service, reinstall
};

#[tokio::main]
async fn main() {
    let check_install_result:bool;
    println!("{}","✨ Checking if zrok is installed...".yellow());
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

    if check_install_result {
        
        let theme = ColorfulTheme::default();

        loop {
            clearscreen::clear().unwrap();
            println!("{}", "=== [Autorok] ===".purple());

            let items = vec![
                "Enable environment", 
                "Disable environment",
                "Display all service",
                "Share service",
                "Join service",
                "Create service",
                "Delete service",
                "Reinstall zrok",
                "Exit"
            ];
            let selected = Select::with_theme(&theme)
                .items(&items)
                .default(0)
                .interact()
                .unwrap();

            match selected {
                0 => {
                    clearscreen::clear().unwrap();
                    enviroment::enable()
                },
                1 => {
                    clearscreen::clear().unwrap();
                    enviroment::disable()
                },
                2 => {
                    
                    clearscreen::clear().unwrap();
                    println!("{}", "=== Display All Service ===".purple());
                    display_all_service::new();
                    println!("{}", "Press any key to continue...".yellow());
                    let mut dummy = String::new();
                    io::stdin().read_line(&mut dummy).unwrap();
                }
                3 => {
                    clearscreen::clear().unwrap();
                    share_service::new();
                }
                4 => {
                    clearscreen::clear().unwrap();
                    join_service::new();
                }
                5 => {
                    clearscreen::clear().unwrap();
                    create_service::new();
                }
                6 => {
                    clearscreen::clear().unwrap();
                    delete_service::new();
                }
                7 => {
                    clearscreen::clear().unwrap();
                    let reinstall_result = reinstall::new().await;
                    if reinstall_result == false {
                        println!("{}", "❌ Reinstall zrok failed".red());
                        println!("{}","👋 Exited autorok.".yellow());
                        println!("{}", "Press any key to continue...".yellow());
                        let mut dummy = String::new();
                        io::stdin().read_line(&mut dummy).unwrap();
                        
                        break;
                    }else{
                        println!("{}", "Press any key to continue...".yellow());
                        let mut dummy = String::new();
                        io::stdin().read_line(&mut dummy).unwrap();
                    }
                    
                }
                8 => {
                    clearscreen::clear().unwrap();
                    println!("{}","👋 Exited autorok.".yellow());
                    println!("{}", "Press any key to continue...".white());
                    let mut dummy = String::new();
                    io::stdin().read_line(&mut dummy).unwrap();
                    break;
                }
                _ => {
                    println!("{}","Invalid option!".yellow());
                }
            }
            
        }
        
    }
}
