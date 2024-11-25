use std::{num::ParseIntError, ops::Range};

use decl::{parse_file_name, parse_func, parse_table};
use logos::Logos as _;
use thiserror::Error;
use util::PResult;

use crate::format::ByteCodeFile;

use super::lexer::Token;

mod decl;
mod instr;
mod mark;
mod util;

const DECL_HEADERS: &str = "one of 'func', 'type' or 'file'";

pub fn parse_text_file(text: &str) -> PResult<'_, ByteCodeFile> {
    let mut file = ByteCodeFile::default();
    let mut lex = Token::lexer(text).spanned().peekable();

    while let Some(next) = lex.next() {
        match next {
            (Ok(Token::Func), _) => {
                let (id, func) = parse_func(&mut lex)?;
                file.funcs.insert(id, func);
            }
            (Ok(Token::Type), _) => {
                let (id, table) = parse_table(&mut lex)?;
                file.tables.insert(id, table);
            }
            (Ok(Token::File), _) => {
                let (id, name) = parse_file_name(&mut lex)?;
                file.file_names.insert(id, name);
            }
            (Err(()), range) => return Err(ParseError::LexError { range }),
            (Ok(found), range) => {
                return Err(ParseError::UnexpectedToken {
                    range,
                    found,
                    expected: DECL_HEADERS,
                })
            }
        }
    }
    Ok(file)
}

#[derive(Error, Debug)]
pub enum ParseError<'src> {
    #[error("Unable to lex token")]
    LexError { range: Range<usize> },

    #[error("Unexpected token {found:?}")]
    UnexpectedToken {
        range: Range<usize>,
        found: Token<'src>,
        expected: &'static str,
    },
    #[error("Expected {expected}, but there was no more input")]
    UnexpectedEndOfInput { expected: &'static str },

    #[error("{}", err)]
    ParseIntError {
        err: ParseIntError,
        range: Range<usize>,
    },

    #[error("EOI")]
    ImpliedEnd,
}
