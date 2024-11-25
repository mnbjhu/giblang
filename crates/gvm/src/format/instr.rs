use super::literal::Literal;

#[derive(Debug)]
pub enum ByteCode {
    Push(Literal),
    Copy,
    Pop,
    Print,
    Panic,
    Construct { id: u32, len: u32 },
    Dyn(u64),
    Call(u32),
    DynCall(u32),
    Return,
    Index(u32),
    SetIndex(u32),

    VecGet,
    VecSet,
    VecPush,
    VecPop,
    VecPeak,
    VecInsert,
    VecRemove,
    VecLen,

    NewLocal(u32),
    GetLocal(u32),
    SetLocal(u32),
    Param(u32),
    Je(u32),
    Jne(u32),
    Jmp(u32),

    Mul,
    Div,
    Add,
    Sub,
    Mod,

    Lt,
    Gt,
    Lte,
    Gte,

    Eq,
    Neq,

    Or,
    And,
    Not,
    Match(u32),
    Clone,
}
