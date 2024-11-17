use clap::Parser as _;
use cli::Command;

mod check;
mod cli;
mod db;
mod go;
mod ir;
mod item;
mod lexer;
mod lsp;
mod parser;
mod range;
mod resolve;
mod run;
mod ty;
mod util;

#[tokio::main]
async fn main() {
    Command::parse().run().await;
}
