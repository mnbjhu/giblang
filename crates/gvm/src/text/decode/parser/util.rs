use std::{iter::Peekable, num::ParseIntError, ops::Range, str::FromStr};

use logos::SpannedIter;

use crate::{format::literal::Literal, text::decode::lexer::Token};

use super::ParseError;

pub type PResult<'src, T> = Result<T, ParseError<'src>>;

pub type Lex<'src> = Peekable<SpannedIter<'src, Token<'src>>>;

pub type Spanned<T> = (T, Range<usize>);

pub fn expect_next<'src>(
    lex: &mut Lex<'src>,
    expected: &'static str,
) -> PResult<'src, Spanned<Token<'src>>> {
    match lex.next() {
        Some((Ok(tok), range)) => Ok((tok, range)),
        Some((Err(()), range)) => Err(ParseError::LexError { range }),
        None => Err(ParseError::UnexpectedEndOfInput { expected }),
    }
}

pub fn expect_num<'src, T: FromStr<Err = ParseIntError>>(
    lex: &mut Lex<'src>,
    expected: &'static str,
) -> PResult<'src, T> {
    let (next, range) = expect_next(lex, expected)?;
    if let Token::Int(text) = next {
        Ok(text
            .parse()
            .map_err(|err| ParseError::ParseIntError { err, range })?)
    } else {
        Err(ParseError::UnexpectedToken {
            range,
            found: next,
            expected,
        })
    }
}

pub fn expect_string<'src>(
    lex: &mut Lex<'src>,
    expected: &'static str,
) -> PResult<'src, Spanned<&'src str>> {
    let (next, range) = expect_next(lex, expected)?;
    if let Token::String(text) = next {
        Ok((text, range))
    } else {
        Err(ParseError::UnexpectedToken {
            range,
            found: next,
            expected,
        })
    }
}

pub fn parse_literal<'src>(lex: &mut Lex<'src>) -> PResult<'src, Literal> {
    const EXPECTED_LITERALS: &str = "String, Int, Float, Char or Bool";
    let (next, range) = expect_next(lex, EXPECTED_LITERALS)?;
    match next {
        Token::String(value) => Ok(Literal::String(value.to_string())),
        Token::Char(value) => Ok(Literal::Char(value)),
        Token::Int(value) => Ok(Literal::Int(value.parse().unwrap())),
        Token::Float(value) => Ok(Literal::Float(value.parse().unwrap())),
        Token::True => Ok(Literal::Bool(true)),
        Token::False => Ok(Literal::Bool(false)),
        _ => Err(ParseError::UnexpectedToken {
            range,
            found: next,
            expected: EXPECTED_LITERALS,
        }),
    }
}

pub fn assert_not_decl<'src>(lex: &mut Lex<'src>) -> PResult<'src, ()> {
    let next = lex.peek();
    if let Some((Ok(tok), _)) = next {
        if tok.is_decl() {
            return Err(ParseError::ImpliedEnd);
        }
    } else if next.is_none() {
        return Err(ParseError::ImpliedEnd);
    }
    Ok(())
}
