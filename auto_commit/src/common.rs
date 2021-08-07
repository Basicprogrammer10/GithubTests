/// Remove quotes from a string
///
/// Ex: "Hello" -> Hello
pub fn remove_quotes(s: String) -> String {
    let mut s = s.to_string();
    if s.starts_with("\"") {
        s = s[1..].to_string();
    }
    if s.ends_with("\"") {
        s = s[..s.len() - 1].to_string();
    }
    s
}
