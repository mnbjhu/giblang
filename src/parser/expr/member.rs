use chumsky::{primitive::just, IterParser, Parser};

use crate::{
    lexer::token::punct,
    parser::common::{ident::spanned_ident_parser, optional_newline::optional_newline},
    util::Spanned,
    AstParser,
};

use super::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct MemberCall {
    pub rec: Spanned<Box<Expr>>,
    pub name: Spanned<String>,
    pub args: Vec<Spanned<Expr>>,
}

pub fn member_call_parser<'tokens, 'src: 'tokens>(
    expr: AstParser!(Expr),
) -> AstParser!(CallAccess) {
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

    name.then(args)
        .map(|(name, args)| CallAccess { name, args })
}

#[derive(Clone, PartialEq, Debug)]
pub struct CallAccess {
    pub name: Spanned<String>,
    pub args: Vec<Spanned<Expr>>,
}
