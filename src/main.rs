use db::lazy::watch_test;
// use cli::Command;

// mod check;
// mod cli;
pub mod db;
mod lexer;
pub mod parser;
// pub mod project;
// mod resolve;
// mod ty;
mod util;

fn main() -> ! {
    watch_test();
    // Command::parse().run();
}
