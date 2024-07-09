pub mod build;
pub mod exports;

use build::build;
use exports::exports;

#[derive(Debug, clap::Parser)]
pub enum Command {
    /// Build
    Build {
        /// The path to the source file
        path: String,
    },
    Exports,
}

impl Command {
    pub fn run(&self) {
        match self {
            Command::Build { path } => build(path),
            Command::Exports => exports(),
        }
    }
}
