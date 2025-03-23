use crate::command::{Command, Outcome};

pub struct Cd;

impl Command for Cd {
    fn execute(&self, args: &[&str]) -> crate::command::Outcome {
        let Some(dir) = args.first() else {
            return Outcome::error("cd: missing argument");
        };

        // Only works for `simple` ~ cases
        // not `~/Downloads` or complex interpolations
        if dir == &"~" {
            let home = std::env::var("HOME").unwrap();
            std::env::set_current_dir(home).unwrap();
            return Outcome::default();
        }

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
