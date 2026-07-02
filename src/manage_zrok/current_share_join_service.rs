use std::io::{self, Write, BufReader, BufRead};
use colored::Colorize;
use std::process::{Command, Stdio};
use serde_json::Value;
use tabled::{Tabled, Table, settings::Style};
use dashmap::DashMap;
use once_cell::sync::Lazy;

use crate::manage_zrok::all_service::{self, ShareInfo};

pub static CURRENT_SHARE_SERVICE: Lazy<DashMap<usize, ShareInfo>> = Lazy::new(DashMap::new);
pub static CURRENT_JOIN_SERVICE: Lazy<DashMap<usize, ShareInfo>> = Lazy::new(DashMap::new);


pub fn new() -> anyhow::Result<()> {
    let all_service = all_service::new();

    if all_service.len() == 0 {
        println!("{}", "❌ No services found.".red());
        return Ok(());
    }

    if CURRENT_SHARE_SERVICE.len() > 0 {
        println!("{}", "[>] Current Share Service".green());
        let current_share_service: Vec<ShareInfo> = CURRENT_SHARE_SERVICE.iter().map(|item| item.value().clone()).collect();
        let mut table = Table::new(&current_share_service);
        table.with(Style::rounded());
        println!("{}", table);
    }else{
        println!("{}", "=> No current share service found.".yellow());
    }
    
    if CURRENT_JOIN_SERVICE.len() > 0 {
        println!("{}", "[>] Current Share Service".green());
        let current_join_service: Vec<ShareInfo> = CURRENT_JOIN_SERVICE.iter().map(|item| item.value().clone()).collect();
        let mut table = Table::new(&current_join_service);
        table.with(Style::rounded());
        println!("{}", table);
    }else{
        println!("{}", "=> No current join service found.".yellow());
    }
    

    return Ok(());
    
}