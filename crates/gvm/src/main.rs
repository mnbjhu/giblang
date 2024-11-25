use clap::Parser;
use cli::Command;

mod binary;
mod cli;
mod format;
mod text;
mod vm;

fn main() {
    Command::parse().run()
}
