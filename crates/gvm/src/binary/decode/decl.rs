use std::{collections::HashMap, iter::Peekable};

use crate::format::{func::FuncDef, ByteCodeFile};

use super::{
    op::decode_code,
    util::{decode_big, decode_small, decode_tiny},
};

pub fn decode_func<T: Iterator<Item = u8>>(bytes: &mut Peekable<T>, into: &mut ByteCodeFile) {
    let id = decode_small(bytes);
    let args = decode_small(bytes);
    let name_len = decode_small(bytes);
    let mut name = String::new();
    for _ in 0..name_len {
        name.push(bytes.next().unwrap() as char);
    }
    let line = decode_tiny(bytes);
    let char = decode_tiny(bytes);
    let file = decode_small(bytes);
    let pos = (line, char);
    let mut func = FuncDef {
        name,
        args,
        body: Vec::new(),
        pos,
        file,
        marks: Vec::new(),
    };

    let marks_len = decode_small(bytes);
    for _ in 0..marks_len {
        let index = decode_small(bytes);
        let line = decode_tiny(bytes);
        let col = decode_tiny(bytes);
        func.marks.push((index as usize, (line, col)));
    }

    while let Some(bc) = decode_code(bytes) {
        func.body.push(bc);
    }
    into.funcs.insert(id, func);
}

pub fn decode_table<T: Iterator<Item = u8>>(bytes: &mut Peekable<T>, into: &mut ByteCodeFile) {
    let id = decode_big(bytes);
    let len = decode_small(bytes);
    let mut items = HashMap::new();
    for _ in 0..len {
        let key = decode_small(bytes);
        let value = decode_small(bytes);
        items.insert(key, value);
    }
    into.tables.insert(id, items);
}

pub fn decode_file_name<T: Iterator<Item = u8>>(bytes: &mut Peekable<T>, into: &mut ByteCodeFile) {
    let id = decode_small(bytes);
    let len = decode_small(bytes);
    let mut name = String::new();
    for _ in 0..len {
        name.push(bytes.next().unwrap() as char);
    }
    into.file_names.insert(id, name);
}
