pub mod build;

use build::build;

#[derive(Debug, clap::Parser)]
pub enum Command {
    /// Build
    Build {
        /// The path to the source file
        path: String,
    },
}

impl Command {
    pub fn run(&self) {
        match self {
            Command::Build { path } => build(path),
        }
    }
}
