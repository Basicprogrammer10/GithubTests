use chrono::prelude::*;
use rand::Rng;
use std::process::Command;

/// Remove quotes from a string
///
/// Ex: "Hello" -> Hello
pub fn remove_quotes(s: String) -> String {
    let mut s = s;
    if s.starts_with('"') {
        s = s[1..].to_string();
    }
    if s.ends_with('"') {
        s = s[..s.len() - 1].to_string();
    }
    s
}

/// Replace the placeholders in the configuration file with the values
///
/// Replaces $2 with the current date (YYYY-MM-DD)
///
/// Replaces $1 with the run count (COMMING SOON)
/// ## Example
/// ```rust
/// // Example Output
/// assert_eq!(replace_placeholders("$1_$2"), "13_2020-8-6");
/// ```
pub fn replace_values(mut message: String) -> String {
    message = message.replace("$2", &Utc::today().format("%Y-%m-%d").to_string());
    message = message.replace("$3", &rand::thread_rng().gen_range(0..100000).to_string());
    message
}

/// Execute a command...
/// ## Example
/// ```rust
/// run(&["ls", "-l"]);
/// ```
pub fn run(command: &[&str]) {
    Command::new("sh") // Use cmd or something for windows
        .args(command)
        .output()
        .expect("failed to execute process");
}
