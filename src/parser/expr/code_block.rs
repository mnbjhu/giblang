use chumsky::{primitive::just, recovery::via_parser, IterParser, Parser};

use crate::{
    lexer::token::{newline, punct},
    parser::{common::optional_newline::optional_newline, stmt::Stmt, top_recovery},
    util::Spanned,
    AstParser,
};

pub type CodeBlock = Vec<Spanned<Stmt>>;

pub fn code_block_parser<'tokens, 'src: 'tokens>(stmt: AstParser!(Stmt)) -> AstParser!(CodeBlock) {
    stmt.map_with(|s, e| Some((s, e.span())))
        .recover_with(via_parser(top_recovery().map(|()| None)))
        .separated_by(just(newline()))
        .collect()
        .map(|stmts: Vec<_>| stmts.into_iter().flatten().collect())
        .delimited_by(
            just(punct('{')).then(optional_newline()),
            optional_newline().then(just(punct('}'))),
        )
}
