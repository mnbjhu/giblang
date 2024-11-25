use std::collections::HashMap;

use crate::{
    format::{func::FuncDef, table::VTable},
    text::decode::lexer::Token,
};

use super::{
    instr::parse_instr,
    mark::parse_mark,
    util::{assert_not_decl, expect_num, expect_string, Lex, PResult},
    ParseError,
};

pub fn parse_func<'src>(lex: &mut Lex<'src>) -> PResult<'src, (u32, FuncDef)> {
    let id = expect_num(lex, "'id' (u32)")?;
    let args = expect_num(lex, "'arg count' (u32)")?;
    let name = expect_string(lex, "'name' (String)")?.0.to_string();
    let line = expect_num(lex, "'line' (u16)")?;
    let col = expect_num(lex, "'col' (u16)")?;
    let file = expect_num(lex, "'file id' (u32)")?;
    let mut func = FuncDef {
        name,
        args,
        pos: (line, col),
        file,
        body: vec![],
        marks: vec![],
    };
    loop {
        if let Some((Ok(tok), _)) = lex.peek() {
            if let Token::Mark = tok {
                lex.next();
                let (index, pos) = parse_mark(lex)?;
                func.marks.push((index as usize, pos));
                continue;
            } else if tok.is_decl() {
                break;
            }
        }
        match parse_instr(lex) {
            Ok(instr) => func.body.push(instr),
            Err(ParseError::ImpliedEnd) => break,
            Err(err) => return Err(err),
        }
    }
    Ok((id, func))
}

pub fn parse_table<'src>(lex: &mut Lex<'src>) -> Result<(u64, VTable), ParseError<'src>> {
    let id = expect_num(lex, "'id' (u64)")?;
    let mut items = HashMap::new();
    loop {
        if assert_not_decl(lex).is_err() {
            break;
        }
        let key = expect_num(lex, "'key' (u32)")?;
        let val = expect_num(lex, "'val' (u32)")?;
        items.insert(key, val);
    }
    Ok((id, items))
}

pub fn parse_file_name<'src>(lex: &mut Lex<'src>) -> Result<(u32, String), ParseError<'src>> {
    let id = expect_num(lex, "'id' (u32)")?;
    let name = expect_string(lex, "'name' (String)")?.0.to_string();
    Ok((id, name))
}
