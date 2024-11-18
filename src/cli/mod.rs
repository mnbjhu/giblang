pub mod build;
pub mod byte_code_text;
pub mod debug;
pub mod exports;
pub mod file_tree;
pub mod fmt;
pub mod lex;
pub mod module_tree;
pub mod parse;
pub mod run;

use std::path::PathBuf;

use build::build;
use byte_code_text::byte_code_text;
use debug::debug;
use exports::exports;
use file_tree::file_tree;
use fmt::fmt;
use lex::lex;
use module_tree::module_tree;
use parse::parse;
use run::run;

use crate::lsp::main_loop;

#[derive(Debug, clap::Parser)]
pub enum Command {
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

    /// Builds the project
    Build,

    /// Runs the project
    Run,

    /// Runs the project with debug traces
    Debug,

    /// Shows a tree of the exports
    Exports,

    /// Start the language server
    Lsp,

    /// Show the included files
    FileTree,

    /// Show the module tree
    ModuleTree,

    /// Format a file
    Fmt {
        /// The path to the source file
        path: PathBuf,
    },

    /// Execute bytecode text format
    ByteCodeText {
        /// The path to the source file
        path: PathBuf,
    },
}

impl Command {
    pub async fn run(&self) {
        match self {
            Command::Parse { path } => parse(path),
            Command::Exports => exports(),
            Command::Build => build(),
            Command::Lsp => main_loop().await,
            Command::FileTree => file_tree(),
            Command::ModuleTree => module_tree(),
            Command::Lex { path } => lex(path),
            Command::Fmt { path } => fmt(path),
            Command::Run => run(),
            Command::ByteCodeText { path } => byte_code_text(path),
            Command::Debug => debug(),
        }
    }
}
