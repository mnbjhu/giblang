use clap::Parser;
use cli::Command;

pub mod check;
pub mod cli;
pub mod fs;
pub mod lexer;
pub mod parser;
pub mod project;
pub mod resolve;
pub mod ty;
pub mod util;

fn main() {
    Command::parse().run();
}
