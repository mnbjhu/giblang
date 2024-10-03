// use db::lazy::watch_test;
// use cli::Command;

use lsp::main_loop;

// mod check;
// mod cli;
pub mod db;
mod lexer;
pub mod lsp;
pub mod parser;
pub mod range;
// pub mod project;
// mod resolve;
// mod ty;
mod util;

#[tokio::main]
async fn main() {
    main_loop().await;
    // watch_test();
    // Command::parse().run();
}
