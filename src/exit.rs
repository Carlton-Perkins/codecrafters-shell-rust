use crate::command::{Command, Outcome};

pub struct Exit;

impl Command for Exit {
    fn execute(&self, _args: &[&str]) -> Outcome {
        Outcome { exit: true }
    }
}
