use crate::command::{Command, Outcome};

#[derive(Clone)]
pub struct Echo;

impl Command for Echo {
    fn execute(&self, args: &[&str]) -> Outcome {
        println!("{}", args.join(" "));
        Outcome::default()
    }
}
