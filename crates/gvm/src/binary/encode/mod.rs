use decl::{get_file_name_bytes, get_table_bytes};

use crate::format::ByteCodeFile;

mod decl;
mod op;

pub fn encode_program(prog: &ByteCodeFile) -> Vec<u8> {
    let mut bytes = vec![];
    for (id, name) in &prog.file_names {
        bytes.extend_from_slice(&get_file_name_bytes(*id, name));
    }
    for (id, items) in &prog.tables {
        bytes.extend_from_slice(&get_table_bytes(*id, items));
    }
    for (id, func) in &prog.funcs {
        bytes.extend_from_slice(&func.get_bytes(*id));
    }
    bytes
}
