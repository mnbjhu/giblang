use clap::Parser;
use cli::Command;

mod check;
mod cli;
mod lexer;
mod parser;
mod project;
mod resolve;
mod ty;
mod util;

fn main() {
    Command::parse().run();
}
