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

#[cfg(test)]
mod tests {
    use crate::cli::build::build;

    #[test]
    fn test_build() {
        build();
    }
}
