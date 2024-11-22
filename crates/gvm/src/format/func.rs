use super::{instr::ByteCode, span::ByteCodeSpan};

pub struct FuncDef {
    pub name: String,
    pub args: u32,
    pub pos: (u16, u16),
    pub file: u32,
    pub body: Vec<ByteCode>,
    pub marks: Vec<(usize, ByteCodeSpan)>,
}
