use clap::Parser;
use cli::Command;

mod check;
mod cli;
mod lexer;
mod parser;
pub mod project;
mod resolve;
mod resolve_types;
mod ty;
mod util;

fn main() {
    Command::parse().run();
}
