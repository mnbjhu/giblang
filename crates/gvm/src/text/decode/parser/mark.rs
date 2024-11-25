use super::util::{expect_num, Lex, PResult};

pub fn parse_mark<'src>(lex: &mut Lex<'src>) -> PResult<'src, (u32, (u16, u16))> {
    Ok((
        expect_num(lex, "'index' (u32)")?,
        (
            expect_num(lex, "'line' (u16)")?,
            expect_num(lex, "'col' (u16)")?,
        ),
    ))
}
