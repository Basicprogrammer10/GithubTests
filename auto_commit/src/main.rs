use simple_config_parser::config::Config;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;

extern crate chrono;

mod arg_parse;
mod common;

pub static VERSION: &str = "0.0.1";

fn main() {
    println!("[*] Starting Auto Commit V{}", VERSION);
    let args: Vec<String> = env::args().collect();
    let debug = args.iter().any(|i| i == "--debug");

    // Get github token
    // If no token is provided, PANIC!
    let token = match arg_parse::get_arg_value(&args, "--token") {
        Some(v) => v.to_string(),
        None => panic!("No token defined, Use --token <value>"),
    };

    // Get github repo
    let repo = match arg_parse::get_arg_value(&args, "--repo") {
        Some(v) => v.to_string(),
        None => panic!("No repo defined, Use --repo <value>"),
    };

    // Load config //
    let cfg = Config::new(Some("config.ini"));

    // Get values from config file
    let message = cfg
        .get("message")
        .unwrap_or_else(|| "Auto-Commit: #$1".to_string());
    let content = cfg
        .get("content")
        .unwrap_or_else(|| "Nose - $3".to_string());
    let username = cfg
        .get("username")
        .unwrap_or_else(|| "dailyCommit".to_string());
    let file_name = cfg.get("file").unwrap_or_else(|| "nose".to_string());
    let email = cfg.get("email").unwrap_or_else(|| "".to_string());

    // Modify Files //
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(common::remove_quotes(file_name))
        .expect("Error opening file");

    writeln!(file, "{}", common::replace_values(content)).expect("Error writing to file");

    // Setup Git //

    // Set UserName
    if debug { println!("[*] Setting Username: {}", username); }
    common::run(&["git", "config", "--global", "user.name", &username]);

    // Set Email
    if debug { println!("[*] Setting Email: {}", email); }
    common::run(&["git", "config", "--global", "user.email", &email]);

    // Set repo (Repo , Token)
    if debug { println!("[*] Setting Repo / Token: ({} / haha not telling ya)", repo); }
    common::run(&[&format!(
        "git remote set-url origin https://x-access-token:{}@github.com/{}",
        token, repo
    )]);

    // Commit
    if debug { println!("[*] Committing"); }
    common::run(&["git", "commit", "-am", &message]);

    // Push
    if debug { println!("[*] Pushing"); }
    common::run(&["git", "push", "origin", "HEAD:master"]);
}
