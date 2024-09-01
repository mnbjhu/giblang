use std::{env::set_current_dir, fs, process::Command};

use crate::project::Project;

pub fn build() {
    let mut project = Project::init_pwd();
    let errors = project.resolve();
    for error in &errors {
        project.print_resolve_error(error);
    }
    if project.check_with_errors() {
        let _ = fs::remove_dir_all("build");
        fs::create_dir("build").expect("Failed to create directory");
        set_current_dir("build").expect("Failed to change directory");
        println!("Pwd: {:?}", std::env::current_dir());
        Command::new("go")
            .arg("mod")
            .arg("init")
            .arg("example.com")
            .status()
            .expect("Failed to init module");
        project.build();
        Command::new("go")
            .arg("build")
            .status()
            .expect("Failed to build");
    }
}
