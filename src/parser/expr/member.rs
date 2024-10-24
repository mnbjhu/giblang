use chumsky::{primitive::just, IterParser, Parser};

use crate::{
    lexer::token::punct,
    parser::common::{ident::spanned_ident_parser, optional_newline::optional_newline},
    util::Spanned,
    AstParser,
};

use super::Expr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemberCall {
    pub rec: Box<Spanned<Expr>>,
    pub name: Spanned<String>,
    pub args: Vec<Spanned<Expr>>,
}

pub fn member_call_parser<'tokens, 'src: 'tokens>(
    atom: AstParser!(Expr),
    expr: AstParser!(Expr),
) -> AstParser!(MemberCall) {
    let args = expr
        .map_with(|ex, e| (ex, e.span()))
        .separated_by(just(punct(',')).padded_by(optional_newline()))
        .allow_trailing()
        .collect()
        .delimited_by(
            just(punct('(')).then(optional_newline()),
            optional_newline().then(just(punct(')'))),
        );

    let name = just(punct('.')).ignore_then(spanned_ident_parser());

    atom.map_with(|ex, e| (ex, e.span()))
        .map(Box::new)
        .then(name)
        .then(args)
        .map(|((rec, name), args)| MemberCall { rec, name, args })
}
