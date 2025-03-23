use std::{collections::HashMap, path::PathBuf};

use crate::command::{into_box, Command, Outcome};

#[derive(Clone)]
pub struct ExternalCommand {
    pub name: String,
    pub path: PathBuf,
}

impl Command for ExternalCommand {
    fn execute(&self, _args: &[&str]) -> Outcome {
        println!("{} is {}", self.name, self.path.display());

        Outcome::default()
    }
}

pub fn path_search(command: &str) -> Option<PathBuf> {
    let path_var = std::env::var("PATH").unwrap_or_default();
    let paths = path_var.split(':');

    for path in paths {
        let dir = std::fs::read_dir(path);
        let Ok(dir) = dir else {
            continue;
        };

        for entry in dir {
            let entry = entry.unwrap();
            let path = entry.path();
            let file_name = entry.file_name().into_string().unwrap();

            if file_name == command {
                return Some(path);
            }
        }
    }

    None
}

pub fn path_lookup(command: &str) -> Option<Box<dyn Command>> {
    let path = path_search(command);
    if path.is_some() {
        let path = path.unwrap();

        let external = into_box(ExternalCommand {
            name: command.to_string(),
            path: path.clone(),
        });
        return Some(external);
    }

    None
}
