use chumsky::{primitive::just, Parser};

use crate::{
    kw,
    lexer::token::punct,
    op,
    parser::{
        common::{
            optional_newline::optional_newline,
            pattern::{pattern_parser, Pattern},
            type_::{type_parser, Type},
        },
        expr::Expr,
    },
    util::Spanned,
    AstParser,
};

#[derive(Clone, PartialEq, Debug, Eq)]
pub struct LetStatement {
    pub pattern: Spanned<Pattern>,
    pub ty: Option<Spanned<Type>>,
    pub value: Spanned<Expr>,
}

pub fn let_parser<'tokens, 'src: 'tokens>(expr: AstParser!(Expr)) -> AstParser!(LetStatement) {
    let ty = just(punct(':'))
        .padded_by(optional_newline())
        .ignore_then(type_parser().map_with(|t, e| (t, e.span())))
        .or_not();
    just(kw!(let))
        .ignore_then(pattern_parser().map_with(|p, e| (p, e.span())))
        .then(ty)
        .then_ignore(just(op!(=)))
        .then(expr.map_with(|e, s| (e, s.span())))
        .map(|((pattern, ty), value)| LetStatement { pattern, ty, value })
}
