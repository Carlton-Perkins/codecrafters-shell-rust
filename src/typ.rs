use crate::{builtins::register_builtins, command::Command};

pub struct Typ {}

impl Command for Typ {
    fn execute(&self, args: &[&str]) -> crate::command::Outcome {
        let builtins = register_builtins();

        let Some(command) = args.first() else {
            eprintln!("No command provided");
            return crate::command::Outcome::default();
        };

        if let Some(_) = builtins.get(command) {
            println!("{command} is a shell builtin");
        } else {
            println!("{command}: not found");
        }

        crate::command::Outcome::default()
    }
}
