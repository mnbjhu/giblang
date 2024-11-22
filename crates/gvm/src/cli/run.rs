use std::{fs::File, io::Read, path::PathBuf};

use clap::Args;

use crate::{binary::decode::decode_file, vm::state::ProgramState};

#[derive(Args)]
pub struct RunCommand {
    /// The bytecode file to run
    path: PathBuf,

    /// Run in debug
    #[clap(short, long)]
    debug: bool,
}

impl RunCommand {
    pub fn run(&self) {
        let mut file = File::open(&self.path).unwrap();
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        let bytecode = decode_file(&mut buf.into_iter().peekable());
        let mut prog = ProgramState::new(&bytecode.funcs, bytecode.tables, bytecode.file_names);
        if self.debug {
            prog.run_debug();
        } else {
            prog.run();
        };
    }
}
