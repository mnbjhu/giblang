use clap::Parser;
use decode::Decode;
use encode::Encode;
use run::RunCommand;

mod decode;
mod encode;
mod run;

/// Giblang Virtual Machine
#[derive(Parser)]
#[command(version, about,  long_about = Some("A stack-based virtual machine for the Giblang programming language"))]
pub enum Command {
    /// Launch the VM
    Run(RunCommand),

    /// Convert from the text format to the binary format
    Encode(Encode),

    /// Convert from the binary format to the text format
    Decode(Decode),
}

impl Command {
    pub fn run(&self) {
        match self {
            Command::Run(cmd) => cmd.run(),
            Command::Encode(cmd) => cmd.run(),
            Command::Decode(cmd) => cmd.run(),
        }
    }
}
