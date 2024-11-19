use std::{fs, path::Path};

use crate::run::{bin::decode_file, state::ProgramState};

pub fn run_vm(path: &Path) {
    let bytes = fs::read(path).unwrap();
    let bytecode = decode_file(&mut bytes.into_iter().peekable());
    let mut prog = ProgramState::new();
    prog.vtables = bytecode.tables;
    prog.run(&bytecode.funcs);
}
