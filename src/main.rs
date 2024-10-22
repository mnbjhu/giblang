use clap::Parser as _;
use cli::Command;

mod item;
mod check;
mod cli;
mod db;
mod lexer;
mod lsp;
mod parser;
mod project;
mod range;
mod resolve;
mod ty;
mod util;

#[tokio::main]
async fn main() {
    Command::parse().run().await;
}
