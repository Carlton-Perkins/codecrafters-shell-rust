use crate::command::{Command, Outcome};

pub struct Cd;

impl Command for Cd {
    fn execute(&self, args: &[&str]) -> crate::command::Outcome {
        let Some(dir) = args.first() else {
            return Outcome::error("cd: missing argument");
        };

        // Validate directory
        let Ok(_) = std::fs::read_dir(dir) else {
            println!("cd: {dir}: No such file or directory");
            return Outcome::default();
        };

        // Set directory
        std::env::set_current_dir(dir).unwrap();

        Outcome::default()
    }
}
