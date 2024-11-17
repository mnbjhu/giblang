use chumsky::{
    primitive::{choice, just},
    recovery::via_parser,
    span::Span as _,
    IterParser, Parser,
};

use crate::{
    lexer::token::punct,
    parser::common::{ident::spanned_ident_parser, optional_newline::optional_newline},
    util::{Span, Spanned},
    AstParser,
};

use super::Expr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Access {
    Member {
        name: Spanned<String>,
        args: Vec<Spanned<Expr>>,
    },
    Field(Spanned<String>),
}

pub fn access_parser<'tokens, 'src: 'tokens>(
    expr: AstParser!(Expr),
    lambda: AstParser!(Expr),
) -> AstParser!(Access) {
    let args = expr
        .map_with(|ex, e| (ex, e.span()))
        .separated_by(just(punct(',')).padded_by(optional_newline()))
        .collect::<Vec<_>>()
        .delimited_by(
            just(punct('(')).then(optional_newline()),
            optional_newline().then(just(punct(')'))),
        )
        .then(lambda.clone().map_with(|ex, e| (ex, e.span())).or_not())
        .map(|(mut args, l)| {
            if let Some(l) = l {
                args.push(l);
            }
            args
        });

    let lambda_only = lambda.map_with(|ex, e| vec![(ex, e.span())]);
    let member = just(punct('.'))
        .ignore_then(spanned_ident_parser())
        .then(lambda_only.or(args))
        .map(|(name, args)| Access::Member { name, args });
    let field = just(punct('.'))
        .ignore_then(spanned_ident_parser())
        .map(Access::Field)
        .recover_with(via_parser(just(punct('.')).map_with(|_, e| {
            Access::Field((String::new(), Span::to_end(&e.span())))
        })));
    choice((member, field))
}

pub fn basic_access_parser<'tokens, 'src: 'tokens>(expr: AstParser!(Expr)) -> AstParser!(Access) {
    let args = expr
        .map_with(|ex, e| (ex, e.span()))
        .separated_by(just(punct(',')).padded_by(optional_newline()))
        .collect::<Vec<_>>()
        .delimited_by(
            just(punct('(')).then(optional_newline()),
            optional_newline().then(just(punct(')'))),
        );

    let member = just(punct('.'))
        .ignore_then(spanned_ident_parser())
        .then(args)
        .map(|(name, args)| Access::Member { name, args });
    let field = just(punct('.'))
        .ignore_then(spanned_ident_parser())
        .map(Access::Field)
        .recover_with(via_parser(just(punct('.')).map_with(|_, e| {
            Access::Field((String::new(), Span::to_end(&e.span())))
        })));
    choice((member, field))
}
