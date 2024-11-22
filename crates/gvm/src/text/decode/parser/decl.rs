use std::collections::HashMap;

use crate::format::{func::FuncDef, table::VTable};

use super::{
    instr::parse_instr,
    util::{assert_not_decl, expect_num, expect_string, Lex},
    ParseError,
};

pub fn parse_func<'src>(lex: &mut Lex<'src>) -> Result<(u32, FuncDef), ParseError<'src>> {
    let id = expect_num(lex, "'id' (u32)")?;
    let args = expect_num(lex, "'arg count' (u32)")?;
    let name = expect_string(lex, "'name' (String)")?.0.to_string();
    let line = expect_num(lex, "'line' (u16)")?;
    let col = expect_num(lex, "'col' (u16)")?;
    let file = expect_num(lex, "'file id' (u32)")?;
    let mut body = vec![];
    loop {
        if assert_not_decl(lex).is_err() {
            break;
        }
        body.push(parse_instr(lex)?);
    }
    Ok((
        id,
        FuncDef {
            name,
            args,
            pos: (line, col),
            file,
            body,
            marks: vec![],
        },
    ))
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
