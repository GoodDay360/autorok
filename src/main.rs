
use colored::Colorize;
use dialoguer::{Select, theme::ColorfulTheme};
use clearscreen;
use std::io::{self};

pub mod manage_zrok;
pub mod utils;

use manage_zrok::{
    check_install, enviroment, all_service, share_service, delete_service, create_service, join_service, reinstall
};



#[tokio::main]
async fn main() -> anyhow::Result<()> {
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
            clearscreen::clear()?;
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
                .interact()?;

            match selected {
                0 => {
                    clearscreen::clear()?;
                    enviroment::enable()
                },
                1 => {
                    clearscreen::clear()?;
                    enviroment::disable()
                },

                2 => {
                    
                    clearscreen::clear()?;
                    println!("{}", "=== Display All Service ===".purple());
                    all_service::new();
                    println!("{}", "Press any key to continue...".yellow());
                    io::stdin().read_line(&mut String::new())?;
                }
                
                3 => {
                    clearscreen::clear()?;
                    share_service::new()?;
                }
                4 => {
                    clearscreen::clear()?;
                    join_service::new()?;
                }
                5 => {
                    clearscreen::clear()?;
                    create_service::new();
                }
                6 => {
                    clearscreen::clear()?;
                    delete_service::new();
                }
                7 => {
                    clearscreen::clear()?;
                    let reinstall_result = reinstall::new().await;
                    if reinstall_result == false {
                        println!("{}", "❌ Reinstall zrok failed".red());
                        println!("{}","👋 Exited autorok.".yellow());
                        println!("{}", "Press any key to continue...".yellow());
                        io::stdin().read_line(&mut String::new())?;
                        
                        break;
                    }else{
                        println!("{}", "Press any key to continue...".yellow());
                        io::stdin().read_line(&mut String::new())?;
                    }
                    
                }
                8 => {
                    clearscreen::clear()?;
                    println!("{}","👋 Exited autorok.".yellow());
                    println!("{}", "Press any key to continue...".white());
                    io::stdin().read_line(&mut String::new())?;
                    break;
                }
                _ => {
                    println!("{}","Invalid option!".yellow());
                }
            }
            
        }
        
    }

    Ok(())
}
