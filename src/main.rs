#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        prompt();

        // Wait for user input
        let input = get_user_input();

        println!("{}: command not found", input);
    }
}

fn get_user_input() -> String {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    input.trim().to_string()
}

fn prompt() {
    print!("$ ");
    io::stdout().flush().unwrap();
}
