use std::{collections::HashMap, iter::Peekable};

use crate::{check::build_state::VTable, lexer::literal::Literal};

use super::{bytecode::ByteCode, state::FuncDef, text::ByteCodeFile};

pub fn decode_file<T: Iterator<Item = u8>>(bytes: &mut Peekable<T>) -> ByteCodeFile {
    let mut funcs = HashMap::new();
    let mut tables = HashMap::new();
    let mut file_names = HashMap::new();
    while let Some(code) = bytes.next() {
        match code {
            0 => {
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
                while let Some(bc) = decode_code(bytes) {
                    if let ByteCode::Mark(line, col) = bc {
                        func.marks.push((func.body.len(), (line, col)));
                    } else {
                        func.body.push(bc);
                    }
                }
                funcs.insert(id, func);
            }
            1 => {
                let id = decode_big(bytes);
                let len = decode_small(bytes);
                let mut items = HashMap::new();
                for _ in 0..len {
                    let key = decode_small(bytes);
                    let value = decode_small(bytes);
                    items.insert(key, value);
                }
                tables.insert(id, items);
            }
            49 => {
                let id = decode_small(bytes);
                let len = decode_small(bytes);
                let mut name = String::new();
                for _ in 0..len {
                    name.push(bytes.next().unwrap() as char);
                }
                file_names.insert(id, name);
            }
            _ => panic!("Invalid byte code header"),
        }
    }
    ByteCodeFile {
        file_names,
        funcs,
        tables,
    }
}

#[allow(clippy::too_many_lines)]
pub fn decode_code<T: Iterator<Item = u8>>(bytes: &mut Peekable<T>) -> Option<ByteCode> {
    if let Some(byte) = bytes.peek() {
        match byte {
            2 => {
                bytes.next();
                Some(ByteCode::Pop)
            }
            3 => {
                bytes.next();
                Some(ByteCode::Print)
            }
            4 => {
                bytes.next();
                Some(ByteCode::Panic)
            }
            5 => {
                bytes.next();
                let id = decode_small(bytes);
                let len = decode_small(bytes);
                Some(ByteCode::Construct { id, len })
            }
            6 => {
                bytes.next();
                let big = decode_big(bytes);
                Some(ByteCode::Dyn(big))
            }
            7 => {
                bytes.next();
                let small = decode_small(bytes);
                Some(ByteCode::Call(small))
            }
            8 => {
                bytes.next();
                let small = decode_small(bytes);
                Some(ByteCode::DynCall(small))
            }
            9 => {
                bytes.next();
                Some(ByteCode::Return)
            }
            10 => {
                bytes.next();
                let small = decode_small(bytes);
                Some(ByteCode::Index(small))
            }
            11 => {
                bytes.next();
                let small = decode_small(bytes);
                Some(ByteCode::SetIndex(small))
            }
            12 => {
                bytes.next();
                Some(ByteCode::VecGet)
            }
            13 => {
                bytes.next();
                Some(ByteCode::VecSet)
            }
            14 => {
                bytes.next();
                Some(ByteCode::VecPush)
            }
            15 => {
                bytes.next();
                Some(ByteCode::VecPop)
            }
            16 => {
                bytes.next();
                Some(ByteCode::VecPeak)
            }
            17 => {
                bytes.next();
                Some(ByteCode::VecInsert)
            }
            18 => {
                bytes.next();
                Some(ByteCode::VecRemove)
            }
            19 => {
                bytes.next();
                Some(ByteCode::VecLen)
            }
            20 => {
                bytes.next();
                let small = decode_small(bytes);
                Some(ByteCode::NewLocal(small))
            }
            21 => {
                bytes.next();
                let small = decode_small(bytes);
                Some(ByteCode::GetLocal(small))
            }
            22 => {
                bytes.next();
                let small = decode_small(bytes);
                Some(ByteCode::SetLocal(small))
            }
            23 => {
                bytes.next();
                let small = decode_small(bytes);
                Some(ByteCode::Param(small))
            }
            24 => {
                bytes.next();
                let small = decode_small(bytes);
                Some(ByteCode::Goto(small))
            }
            25 => {
                bytes.next();
                let small = decode_sign(bytes);
                Some(ByteCode::Je(small))
            }
            26 => {
                bytes.next();
                let small = decode_sign(bytes);
                Some(ByteCode::Jne(small))
            }
            27 => {
                bytes.next();
                let small = decode_sign(bytes);
                Some(ByteCode::Jmp(small))
            }
            28 => {
                bytes.next();
                Some(ByteCode::Add)
            }
            29 => {
                bytes.next();
                Some(ByteCode::Mul)
            }
            30 => {
                bytes.next();
                Some(ByteCode::Sub)
            }
            31 => {
                bytes.next();
                Some(ByteCode::Or)
            }
            32 => {
                bytes.next();
                Some(ByteCode::And)
            }
            33 => {
                bytes.next();
                Some(ByteCode::Not)
            }
            34 => {
                bytes.next();
                Some(ByteCode::Eq)
            }
            35 => {
                bytes.next();
                Some(ByteCode::Neq)
            }
            36 => {
                bytes.next();
                Some(ByteCode::Lt)
            }
            37 => {
                bytes.next();
                Some(ByteCode::Gt)
            }
            38 => {
                bytes.next();
                Some(ByteCode::Lte)
            }
            39 => {
                bytes.next();
                Some(ByteCode::Gte)
            }
            40 => {
                bytes.next();
                let small = decode_small(bytes);
                Some(ByteCode::Match(small))
            }
            41 => {
                bytes.next();
                Some(ByteCode::Clone)
            }
            42 => {
                bytes.next();
                Some(ByteCode::Copy)
            }
            43 => {
                bytes.next();
                let int = decode_small(bytes);
                Some(ByteCode::Push(Literal::Int(int.to_string())))
            }
            44 => {
                bytes.next();
                let float = f32::from_be_bytes([
                    bytes.next().unwrap(),
                    bytes.next().unwrap(),
                    bytes.next().unwrap(),
                    bytes.next().unwrap(),
                ]);
                Some(ByteCode::Push(Literal::Float(float.to_string())))
            }
            45 => {
                bytes.next();
                let len = decode_small(bytes);
                let mut string = String::new();
                for _ in 0..len {
                    string.push(bytes.next().unwrap() as char);
                }
                Some(ByteCode::Push(Literal::String(string)))
            }
            46 => {
                bytes.next();
                let bool = bytes.next().unwrap() != 0;
                Some(ByteCode::Push(Literal::Bool(bool)))
            }
            47 => {
                bytes.next();
                let char = bytes.next().unwrap();
                Some(ByteCode::Push(Literal::Char(char as char)))
            }
            48 => {
                bytes.next();
                let line = decode_tiny(bytes);
                let col = decode_tiny(bytes);
                Some(ByteCode::Mark(line, col))
            }
            _ => None,
        }
    } else {
        None
    }
}

pub fn decode_tiny<T: Iterator<Item = u8>>(iter: &mut Peekable<T>) -> u16 {
    u16::from_be_bytes([iter.next().unwrap(), iter.next().unwrap()])
}
pub fn decode_small<T: Iterator<Item = u8>>(iter: &mut Peekable<T>) -> u32 {
    u32::from_be_bytes([
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    ])
}

pub fn decode_sign<T: Iterator<Item = u8>>(iter: &mut Peekable<T>) -> i32 {
    i32::from_be_bytes([
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    ])
}

pub fn decode_big<T: Iterator<Item = u8>>(iter: &mut Peekable<T>) -> u64 {
    u64::from_be_bytes([
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    ])
}

impl ByteCode {
    pub fn get_code(&self) -> u8 {
        match self {
            ByteCode::Pop => 2,
            ByteCode::Print => 3,
            ByteCode::Panic => 4,
            ByteCode::Construct { .. } => 5,
            ByteCode::Dyn(_) => 6,
            ByteCode::Call(_) => 7,
            ByteCode::DynCall(_) => 8,
            ByteCode::Return => 9,
            ByteCode::Index(_) => 10,
            ByteCode::SetIndex(_) => 11,
            ByteCode::VecGet => 12,
            ByteCode::VecSet => 13,
            ByteCode::VecPush => 14,
            ByteCode::VecPop => 15,
            ByteCode::VecPeak => 16,
            ByteCode::VecInsert => 17,
            ByteCode::VecRemove => 18,
            ByteCode::VecLen => 19,
            ByteCode::NewLocal(_) => 20,
            ByteCode::GetLocal(_) => 21,
            ByteCode::SetLocal(_) => 22,
            ByteCode::Param(_) => 23,
            ByteCode::Goto(_) => 24,
            ByteCode::Je(_) => 25,
            ByteCode::Jne(_) => 26,
            ByteCode::Jmp(_) => 27,
            ByteCode::Add => 28,
            ByteCode::Mul => 29,
            ByteCode::Sub => 30,
            ByteCode::Or => 31,
            ByteCode::And => 32,
            ByteCode::Not => 33,
            ByteCode::Eq => 34,
            ByteCode::Neq => 35,
            ByteCode::Lt => 36,
            ByteCode::Gt => 37,
            ByteCode::Lte => 38,
            ByteCode::Gte => 39,
            ByteCode::Match(_) => 40,
            ByteCode::Clone => 41,
            ByteCode::Copy => 42,
            ByteCode::Push(lit) => match lit {
                Literal::Int(_) => 43,
                Literal::Float(_) => 44,
                Literal::String(_) => 45,
                Literal::Bool(_) => 46,
                Literal::Char(_) => 47,
            },
            ByteCode::Mark(_, _) => 48,
        }
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        match self {
            ByteCode::Construct { id, len } => {
                let mut bytes = vec![self.get_code()];
                bytes.extend_from_slice(&id.to_be_bytes());
                bytes.extend_from_slice(&len.to_be_bytes());
                bytes
            }
            ByteCode::Copy
            | ByteCode::Pop
            | ByteCode::Print
            | ByteCode::Panic
            | ByteCode::Return
            | ByteCode::VecGet
            | ByteCode::VecSet
            | ByteCode::VecPush
            | ByteCode::VecPop
            | ByteCode::VecPeak
            | ByteCode::VecInsert
            | ByteCode::VecRemove
            | ByteCode::Clone
            | ByteCode::Add
            | ByteCode::Mul
            | ByteCode::Sub
            | ByteCode::Or
            | ByteCode::And
            | ByteCode::Not
            | ByteCode::Eq
            | ByteCode::Neq
            | ByteCode::Lt
            | ByteCode::Gt
            | ByteCode::Lte
            | ByteCode::Gte
            | ByteCode::VecLen => {
                vec![self.get_code()]
            }
            ByteCode::Push(lit) => {
                let mut bytes = vec![self.get_code()];
                match lit {
                    Literal::Int(i) => {
                        let int: i32 = i.parse().unwrap();
                        bytes.extend_from_slice(&int.to_be_bytes());
                    }
                    Literal::Float(f) => {
                        let float: f32 = f.parse().unwrap();
                        bytes.extend_from_slice(&float.to_be_bytes());
                    }
                    Literal::String(s) => {
                        bytes.extend_from_slice(&(s.len() as u32).to_be_bytes());
                        bytes.extend_from_slice(s.as_bytes());
                    }
                    Literal::Bool(b) => {
                        bytes.push(u8::from(*b));
                    }
                    Literal::Char(c) => {
                        bytes.push(*c as u8);
                    }
                }
                bytes
            }
            ByteCode::Dyn(big) => {
                let mut bytes = vec![self.get_code()];
                bytes.extend_from_slice(&big.to_be_bytes());
                bytes
            }
            ByteCode::Call(small)
            | ByteCode::DynCall(small)
            | ByteCode::Index(small)
            | ByteCode::Match(small)
            | ByteCode::SetIndex(small)
            | ByteCode::NewLocal(small)
            | ByteCode::GetLocal(small)
            | ByteCode::SetLocal(small)
            | ByteCode::Param(small)
            | ByteCode::Goto(small) => {
                let mut bytes = vec![self.get_code()];
                bytes.extend_from_slice(&small.to_be_bytes());
                bytes
            }
            ByteCode::Jmp(sign) | ByteCode::Jne(sign) | ByteCode::Je(sign) => {
                let mut bytes = vec![self.get_code()];
                bytes.extend_from_slice(&sign.to_be_bytes());
                bytes
            }
            ByteCode::Mark(line, col) => {
                let mut bytes = vec![self.get_code()];
                bytes.extend_from_slice(&line.to_be_bytes());
                bytes.extend_from_slice(&col.to_be_bytes());
                bytes
            }
        }
    }
}

impl FuncDef {
    pub fn get_bytes(&self, id: u32) -> Vec<u8> {
        let mut bytes = vec![0];
        bytes.extend_from_slice(&id.to_be_bytes());
        bytes.extend_from_slice(&self.args.to_be_bytes());
        let len = self.name.len() as u32;
        bytes.extend_from_slice(&len.to_be_bytes());
        bytes.extend_from_slice(self.name.as_bytes());
        bytes.extend_from_slice(&self.pos.0.to_be_bytes());
        bytes.extend_from_slice(&self.pos.1.to_be_bytes());
        bytes.extend_from_slice(&self.file.to_be_bytes());
        for bc in &self.body {
            bytes.extend_from_slice(&bc.get_bytes());
        }
        bytes
    }
}

pub fn get_table_bytes(id: u64, items: &HashMap<u32, u32>) -> Vec<u8> {
    let mut bytes = vec![1];
    bytes.extend_from_slice(&id.to_be_bytes());
    let len: u32 = items.len() as u32;
    bytes.extend_from_slice(&len.to_be_bytes());
    for (k, v) in items {
        bytes.extend_from_slice(&k.to_be_bytes());
        bytes.extend_from_slice(&v.to_be_bytes());
    }
    bytes
}

pub fn get_file_name_bytes(id: u32, name: &str) -> Vec<u8> {
    let mut bytes = vec![49];
    bytes.extend_from_slice(&id.to_be_bytes());
    let len: u32 = name.len() as u32;
    bytes.extend_from_slice(&len.to_be_bytes());
    bytes.extend_from_slice(name.as_bytes());
    bytes
}

pub fn encode_program(
    funcs: &HashMap<u32, FuncDef>,
    tables: &HashMap<u64, VTable>,
    file_names: &HashMap<u32, String>,
) -> Vec<u8> {
    let mut bytes = vec![];
    for (id, name) in file_names {
        bytes.extend_from_slice(&&get_file_name_bytes(*id, name));
    }
    for (id, items) in tables {
        bytes.extend_from_slice(&get_table_bytes(*id, items));
    }
    for (id, func) in funcs {
        bytes.extend_from_slice(&func.get_bytes(*id));
    }
    bytes
}
