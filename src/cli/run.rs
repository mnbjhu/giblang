use std::process::Command;

use super::build::build;

pub fn run() {
    build();
    Command::new("./out")
        .status()
        .expect("Process exited with an error!");
}
