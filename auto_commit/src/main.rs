use simple_config_parser::config::Config;
use std::fs::OpenOptions;
use chrono::prelude::*;
use std::io::Write;
use std::env;

extern crate chrono;

mod arg_parse;
mod common;

pub static VERSION: &str = "0.0.1";

fn main() {
    println!("[*] Starting Auto Commit V{}", VERSION);

    let args: Vec<String> = env::args().collect();
    // let mut debug = args.iter().any(|i| i == "--debug");

    // Get github token
    // If no token is provided, PANIC!
    let token = match arg_parse::get_arg_value(&args, "--token") {
        Some(v) => v.to_string(),
        None => panic!("No token defined, Use --token <value>"),
    };

    // Load config
    let cfg = Config::new(Some("config.ini"));

    // Get values from config file
    let message = cfg.get("message").unwrap_or("Auto-Commit: #$1".to_string());
    let username = cfg.get("username").unwrap_or("dailyCommit".to_string());
    let file_name = cfg.get("file").unwrap_or("nose".to_string());
    let email = cfg.get("email").unwrap_or("".to_string());

    // Modify File and Send Commit
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(common::remove_quotes(file_name))
        .expect("Error opening file");

    writeln!(file, "{}", replace_values(message));
}

pub fn replace_values(message: String) -> String {
    let mut message = message.clone();
    message = message.replace("$2", Utc::today().format("%Y-%m-%d"));
    message
}
