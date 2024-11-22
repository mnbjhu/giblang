use std::iter::Peekable;

use crate::format::{instr::ByteCode, literal::Literal};

use super::util::{decode_big, decode_sign, decode_small, decode_tiny};

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
                let int = decode_sign(bytes);
                Some(ByteCode::Push(Literal::Int(int)))
            }
            44 => {
                bytes.next();
                let float = f32::from_be_bytes([
                    bytes.next().unwrap(),
                    bytes.next().unwrap(),
                    bytes.next().unwrap(),
                    bytes.next().unwrap(),
                ]);
                Some(ByteCode::Push(Literal::Float(float)))
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
