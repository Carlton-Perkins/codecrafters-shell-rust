use std::collections::HashMap;

use crate::{
    cd::Cd,
    command::{into_box, Command},
    echo::Echo,
    exit::Exit,
    pwd::Pwd,
    typ::Typ,
};

pub fn register_builtins() -> HashMap<&'static str, Box<dyn Command>> {
    let mut builtins = HashMap::new();

    builtins.insert("echo", into_box(Echo {}));
    builtins.insert("exit", into_box(Exit {}));
    builtins.insert("type", into_box(Typ {}));
    builtins.insert("pwd", into_box(Pwd {}));
    builtins.insert("cd", into_box(Cd {}));

    builtins
}
