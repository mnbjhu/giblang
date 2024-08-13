use clap::Parser;
use cli::Command;

mod check;
mod cli;
mod lexer;
mod ls;
mod parser;
pub mod project;
mod resolve;
mod ty;
mod util;

#[tokio::main]
async fn main() {
    Command::parse().run().await;
}
