use std::collections::HashMap;

use func::FuncDef;
use table::VTable;

pub mod func;
pub mod instr;
pub mod literal;
pub mod span;
pub mod table;

#[derive(Default)]
pub struct ByteCodeFile {
    pub funcs: HashMap<u32, FuncDef>,
    pub tables: HashMap<u64, VTable>,
    pub file_names: HashMap<u32, String>,
}
