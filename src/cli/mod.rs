pub mod build;
pub mod exports;
pub mod lsp;
pub mod parse;

use build::build;
use exports::exports;
use lsp::lsp;
use parse::parse;

#[derive(Debug, clap::Parser)]
pub enum Command {
    /// Parses a source file
    Parse {
        /// The path to the source file
        path: String,
    },

    /// Builds the project
    Build,

    /// Starts the language server
    Lsp,

    /// Shows a tree of the exports
    Exports,
}

impl Command {
    pub async fn run(&self) {
        match self {
            Command::Parse { path } => parse(path),
            Command::Exports => exports(),
            Command::Build => build(),
            Command::Lsp => lsp().await,
        }
    }
}
