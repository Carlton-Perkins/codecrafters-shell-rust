mod builtins;
mod command;
mod echo;
mod exit;
mod external;
mod typ;

#[allow(unused_imports)]
use std::io::{self, Write};

use external::path_lookup;

fn main() {
    loop {
        prompt();

        // Wait for user input
        let input = get_user_input();

        let segments: Vec<_> = input.split_whitespace().collect();
        let command = segments[0];
        let rest = &segments[1..];

        let mut builtins = builtins::register_builtins();
        let builtin = builtins.remove(command);

        let mut command_to_run = builtin;

        let external = path_lookup(command);
        if command_to_run.is_none() && external.is_some() {
            command_to_run = external;
        }

        if command_to_run.is_none() {
            println!("{}: command not found", command);
            continue;
        }

        let outcome = command_to_run.unwrap().execute(rest);
        if outcome.exit {
            break;
        }
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
