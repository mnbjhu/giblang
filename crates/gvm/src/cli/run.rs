use std::{
    fs::{self, File},
    io::{stdin, Read},
    path::PathBuf,
};

use clap::Args;

use crate::{binary::decode::decode_file, vm::state::ProgramState};

#[derive(Args)]
pub struct RunCommand {
    /// The bytecode file to run (if not provided stdin will be used)
    path: Option<PathBuf>,

    /// Run in debug
    #[clap(short, long)]
    debug: bool,
}

impl RunCommand {
    pub fn run(&self) {
        let bytes = if let Some(input) = &self.path {
            fs::read(input).unwrap()
        } else {
            let mut bytes = vec![];
            stdin().read_to_end(&mut bytes).unwrap();
            bytes
        };
        let bytecode = decode_file(&mut bytes.into_iter().peekable());
        let mut prog = ProgramState::new(&bytecode.funcs, bytecode.tables, bytecode.file_names);
        if self.debug {
            prog.run_debug();
        } else {
            prog.run();
        };
    }
}
