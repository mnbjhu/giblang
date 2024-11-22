use crate::format::{instr::ByteCode, literal::Literal};

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
                        bytes.extend_from_slice(&i.to_be_bytes());
                    }
                    Literal::Float(f) => {
                        let float = f;
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
