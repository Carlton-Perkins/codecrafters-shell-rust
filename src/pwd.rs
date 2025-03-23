use crate::command::{Command, Outcome};

pub struct Pwd;

impl Command for Pwd {
    fn execute(&self, _args: &[&str]) -> Outcome {
        let current_dir = std::env::current_dir().unwrap();
        println!("{}", current_dir.display());
        Outcome::default()
    }
}
