use crate::{format::instr::ByteCode, text::decode::lexer::Token};

use super::{
    util::{expect_num, parse_literal, Lex, PResult},
    ParseError,
};

pub fn parse_instr<'src>(lex: &mut Lex<'src>) -> PResult<'src, ByteCode> {
    match lex.next() {
        Some((Ok(code), range)) => match code {
            Token::Copy => Ok(ByteCode::Copy),
            Token::Push => {
                let lit = parse_literal(lex)?;
                Ok(ByteCode::Push(lit))
            }
            Token::Pop => Ok(ByteCode::Pop),
            Token::Print => Ok(ByteCode::Print),
            Token::Panic => Ok(ByteCode::Panic),
            Token::Construct => {
                let id = expect_num(lex, "'id' (u32)")?;
                let len = expect_num(lex, "'len' (u32)")?;
                Ok(ByteCode::Construct { id, len })
            }
            Token::Call => {
                let id = expect_num(lex, "'id' (u32)")?;
                Ok(ByteCode::Call(id))
            }
            Token::Return => Ok(ByteCode::Return),
            Token::NewLocal => {
                let id = expect_num(lex, "'id' (u32)")?;
                Ok(ByteCode::NewLocal(id))
            }
            Token::GetLocal => {
                let id = expect_num(lex, "'id' (u32)")?;
                Ok(ByteCode::GetLocal(id))
            }
            Token::SetLocal => {
                let id = expect_num(lex, "'id' (u32)")?;
                Ok(ByteCode::SetLocal(id))
            }
            Token::Param => {
                let id = expect_num(lex, "'id' (u32)")?;
                Ok(ByteCode::Param(id))
            }
            Token::Mul => Ok(ByteCode::Mul),
            Token::Add => Ok(ByteCode::Add),
            Token::Sub => Ok(ByteCode::Sub),
            Token::Eq => Ok(ByteCode::Eq),
            Token::Neq => Ok(ByteCode::Neq),
            Token::Not => Ok(ByteCode::Not),
            Token::And => Ok(ByteCode::And),
            Token::Mod => Ok(ByteCode::Mod),
            Token::Div => Ok(ByteCode::Div),
            Token::Or => Ok(ByteCode::Or),
            Token::Match => {
                let id = expect_num(lex, "'id' (u32)")?;
                Ok(ByteCode::Match(id))
            }
            Token::Jmp => {
                let id = expect_num(lex, "'instr' (u32)")?;
                Ok(ByteCode::Jmp(id))
            }
            Token::Je => {
                let id = expect_num(lex, "'instr' (u32)")?;
                Ok(ByteCode::Je(id))
            }
            Token::Jne => {
                let id = expect_num(lex, "'instr' (u32)")?;
                Ok(ByteCode::Jne(id))
            }
            Token::Index => {
                let id = expect_num(lex, "'index' (u32)")?;
                Ok(ByteCode::Index(id))
            }
            Token::SetIndex => {
                let id = expect_num(lex, "'index' (u32)")?;
                Ok(ByteCode::SetIndex(id))
            }
            Token::Gt => Ok(ByteCode::Gt),
            Token::Lt => Ok(ByteCode::Lt),
            Token::Gte => Ok(ByteCode::Gte),
            Token::Lte => Ok(ByteCode::Lte),
            Token::Clone => Ok(ByteCode::Clone),
            Token::VecGet => Ok(ByteCode::VecGet),
            Token::VecSet => Ok(ByteCode::VecSet),
            Token::VecPush => Ok(ByteCode::VecPush),
            Token::VecPop => Ok(ByteCode::VecPop),
            Token::VecLen => Ok(ByteCode::VecLen),
            Token::VecInsert => Ok(ByteCode::VecInsert),
            Token::VecRemove => Ok(ByteCode::VecRemove),
            Token::VecPeak => Ok(ByteCode::VecPeak),
            Token::Dyn => {
                let id = expect_num(lex, "'id' (u64)")?;
                Ok(ByteCode::Dyn(id))
            }
            Token::DynCall => {
                let id = expect_num(lex, "'id' (u32)")?;
                Ok(ByteCode::DynCall(id))
            }
            Token::Func | Token::Type | Token::File => Err(ParseError::ImpliedEnd),
            found => Err(ParseError::UnexpectedToken {
                range: range.clone(),
                found: found.clone(),
                expected: "an instruction",
            }),
        },
        Some((Err(()), range)) => Err(ParseError::LexError {
            range: range.clone(),
        }),
        None => Err(ParseError::ImpliedEnd),
    }
}
