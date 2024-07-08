use chumsky::{primitive::just, Parser};

use crate::{
    kw, op,
    parser::{
        common::{
            optional_newline::optional_newline,
            pattern::{pattern_parser, Pattern},
            type_::{type_parser, Type},
        },
        expr::{expr_parser, Expr},
    },
    util::Spanned,
    AstParser,
};

use super::Stmt;

#[derive(Clone, PartialEq, Debug)]
pub struct LetStatement {
    pub pattern: Spanned<Pattern>,
    pub ty: Option<Spanned<Type>>,
    pub value: Spanned<Expr>,
}

pub fn let_parser<'tokens, 'src: 'tokens>(stmt: AstParser!(Stmt)) -> AstParser!(LetStatement) {
    let ty = just(op!(:))
        .padded_by(optional_newline())
        .ignore_then(type_parser().map_with(|t, e| (t, e.span())))
        .or_not();
    just(kw!(let))
        .ignore_then(pattern_parser().map_with(|p, e| (p, e.span())))
        .then(ty)
        .then_ignore(just(op!(=)))
        .then(expr_parser(stmt).map_with(|e, s| (e, s.span())))
        .map(|((pattern, ty), value)| LetStatement { pattern, ty, value })
}
