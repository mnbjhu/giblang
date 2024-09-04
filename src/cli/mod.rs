pub mod build;
pub mod check;
pub mod exports;
pub mod parse;
pub mod run;

use build::build;
use check::check;
use exports::exports;
use parse::parse;
use run::run;

#[derive(Debug, clap::Parser)]
pub enum Command {
    /// Parses a source file
    Parse {
        /// The path to the source file
        path: String,
    },

    /// Checks the project
    Check,

    /// Builds the project
    Build,

    /// Builds and runs the project
    Run,

    /// Shows a tree of the exports
    Exports,
}

impl Command {
    pub fn run(&self) {
        match self {
            Command::Parse { path } => parse(path),
            Command::Exports => exports(),
            Command::Check => check(),
            Command::Build => build(),
            Command::Run => run(),
        }
    }
}
