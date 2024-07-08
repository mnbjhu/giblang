use clap::Parser;
use cli::Command;

pub mod cli;
pub mod fs;
pub mod lexer;
pub mod parser;
pub mod util;

fn main() {
    Command::parse().run();
}
