use clap::Parser as _;
use cli::Command;

mod check;
mod cli;
pub mod db;
mod lexer;
pub mod lsp;
pub mod parser;
pub mod project;
pub mod range;
mod resolve;
mod ty;
mod util;

#[tokio::main]
async fn main() {
    Command::parse().run().await;
}
