use std::fmt::Display;

use crate::format::instr::ByteCode;

impl Display for ByteCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ByteCode::Push(lit) => write!(f, "push {}", lit),
            ByteCode::Pop => write!(f, "pop"),
            ByteCode::Print => write!(f, "print"),
            ByteCode::Panic => write!(f, "panic"),
            ByteCode::Construct { id, len } => write!(f, "construct {id}, {len}"),
            ByteCode::Index(index) => write!(f, "index {index}"),
            ByteCode::SetIndex(index) => write!(f, "set_index {index}"),
            ByteCode::Call(id) => write!(f, "call {id}"),
            ByteCode::Return => write!(f, "return"),
            ByteCode::NewLocal(id) => write!(f, "new {id}"),
            ByteCode::GetLocal(id) => write!(f, "get {id}"),
            ByteCode::SetLocal(id) => write!(f, "set {id}"),
            ByteCode::Param(id) => write!(f, "param {id}"),
            ByteCode::Goto(line) => write!(f, "goto {line}"),
            ByteCode::Add => write!(f, "add"),
            ByteCode::Mul => write!(f, "mul"),
            ByteCode::Sub => write!(f, "sub"),
            ByteCode::Or => write!(f, "or"),
            ByteCode::And => write!(f, "and"),
            ByteCode::Not => write!(f, "not"),
            ByteCode::Eq => write!(f, "eq"),
            ByteCode::Copy => write!(f, "copy"),
            ByteCode::Je(diff) => write!(f, "je {diff}"),
            ByteCode::Jne(diff) => write!(f, "jne {diff}"),
            ByteCode::Jmp(diff) => write!(f, "jmp {diff}"),
            ByteCode::Match(id) => write!(f, "match {id}"),
            ByteCode::Lt => write!(f, "lt"),
            ByteCode::Gt => write!(f, "gt"),
            ByteCode::Lte => write!(f, "lte"),
            ByteCode::Gte => write!(f, "gte"),
            ByteCode::Neq => write!(f, "neq"),
            ByteCode::Clone => write!(f, "clone"),
            ByteCode::VecGet => write!(f, "vec_get"),
            ByteCode::VecSet => write!(f, "vec_set"),
            ByteCode::VecPush => write!(f, "vec_push"),
            ByteCode::VecPop => write!(f, "vec_pop"),
            ByteCode::VecPeak => write!(f, "vec_peak"),
            ByteCode::VecInsert => write!(f, "vec_insert"),
            ByteCode::VecRemove => write!(f, "vec_remove"),
            ByteCode::VecLen => write!(f, "vec_len"),
            ByteCode::Dyn(id) => write!(f, "dyn {id}"),
            ByteCode::DynCall(id) => write!(f, "dyn_call {id}"),
        }
    }
}
