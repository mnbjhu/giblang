use chumsky::{primitive::just, recovery::via_parser, span::Span as _, Parser as _};

use crate::{
    op,
    parser::{common::optional_newline::optional_newline, expr::Expr},
    util::{Span, Spanned},
    AstParser,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Assign {
    pub refr: Spanned<Expr>,
    pub value: Spanned<Expr>,
}

pub fn assign_parser<'tokens, 'src: 'tokens>(expr: AstParser!(Expr)) -> AstParser!(Assign) {
    let missing_expr = expr
        .clone()
        .map_with(|p, e| (p, e.span()))
        .then_ignore(just(op!(=)))
        .map_with(|refr, e| Assign {
            refr,
            value: (Expr::Error, Span::to_end(&e.span())),
        });

    let valid = expr
        .clone()
        .map_with(|p, e| (p, e.span()))
        .then_ignore(just(op!(=)).padded_by(optional_newline()))
        .then(expr.map_with(|e, s| (e, s.span())))
        .map(|(refr, value)| Assign { refr, value });

    valid.recover_with(via_parser(missing_expr))
}
