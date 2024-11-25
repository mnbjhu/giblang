mod build;
mod fmt;
mod info;
mod lex;
mod parse;
mod run;

use std::path::PathBuf;

use build::build;
use fmt::fmt;
use info::InfoCommand;
use lex::lex;
use parse::parse;
use run::run;

use crate::{dap::start_dap, lsp::main_loop};

#[derive(Debug, clap::Parser)]
pub enum Command {
    /// Builds the project
    Build,

    /// Runs the project
    Run,

    /// Lex the tokens for a file
    Lex {
        /// The path to the source file
        path: String,
    },

    /// Parses a source file
    Parse {
        /// The path to the source file
        path: PathBuf,
    },

    /// Start the language server
    Lsp,

    /// Start the debug adapter
    Dap {
        /// The path to the source file
        path: PathBuf,
    },

    /// Show information about the project
    #[clap(subcommand)]
    Info(InfoCommand),

    /// Format a file
    Fmt {
        /// The path to the source file
        path: PathBuf,
    },
}

impl Command {
    pub async fn run(&self) {
        match self {
            Command::Parse { path } => parse(path),
            Command::Build => build(),
            Command::Lsp => main_loop().await,
            Command::Lex { path } => lex(path),
            Command::Fmt { path } => fmt(path),
            Command::Run => run(),
            Command::Dap { path } => start_dap(path).unwrap(),
            Command::Info(cmd) => cmd.run(),
        }
    }
}
