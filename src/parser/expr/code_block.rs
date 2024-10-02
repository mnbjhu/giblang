use chumsky::{primitive::just, IterParser, Parser};

use crate::{
    lexer::token::{newline, punct},
    parser::{common::optional_newline::optional_newline, stmt::Stmt},
    util::Spanned,
    AstParser,
};

pub type CodeBlock = Vec<Spanned<Stmt>>;

pub fn code_block_parser<'tokens, 'src: 'tokens>(stmt: AstParser!(Stmt)) -> AstParser!(CodeBlock) {
    stmt.map_with(|s, e| (s, e.span()))
        .separated_by(just(newline()))
        .collect()
        .delimited_by(
            just(punct('{')).then(optional_newline()),
            optional_newline().then(just(punct('}'))),
        )
}
