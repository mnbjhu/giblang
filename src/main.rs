use clap::Parser as _;
use cli::Command;

mod check;
mod cli;
mod db;
mod item;
mod lexer;
mod lsp;
mod parser;
mod range;
mod resolve;
mod ty;
mod util;

#[tokio::main]
async fn main() {
    Command::parse().run().await;
}
