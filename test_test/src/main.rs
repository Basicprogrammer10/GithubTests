fn main() {
    println!("{}", greeter("Connor"));
}

fn greeter(name: &str) -> String{
    return format!("Hello, {}!!", name)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_greeter() {
        assert_eq!("Hello, Connor!", greeter("Connor"));
    }
}