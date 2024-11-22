use std::iter::Peekable;

use decl::{decode_file_name, decode_func, decode_table};

use crate::format::ByteCodeFile;

mod decl;
mod op;
mod util;

pub fn decode_file<T: Iterator<Item = u8>>(bytes: &mut Peekable<T>) -> ByteCodeFile {
    let mut file = ByteCodeFile::default();
    while let Some(code) = bytes.next() {
        match code {
            0 => decode_func(bytes, &mut file),
            1 => decode_table(bytes, &mut file),
            49 => decode_file_name(bytes, &mut file),
            _ => panic!("Invalid byte code header"),
        }
    }
    file
}
