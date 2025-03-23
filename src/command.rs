pub trait Command {
    fn execute(&self, args: &[&str]) -> Outcome;
}

#[derive(Debug, Default)]
pub struct Outcome {
    pub exit: bool,
}
impl Outcome {
    pub(crate) fn error(arg: &str) -> Outcome {
        println!("{arg}");

        Outcome::default()
    }
}

pub fn into_box(s: impl Command + 'static) -> Box<dyn Command> {
    Box::new(s)
}
