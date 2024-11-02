use chumsky::{primitive::just, IterParser, Parser};

use crate::{
    lexer::token::punct, parser::common::optional_newline::optional_newline, util::Spanned,
    AstParser,
};

use super::Expr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    pub name: Spanned<Box<Expr>>,
    pub args: Vec<Spanned<Expr>>,
}

pub fn call_parser<'tokens, 'src: 'tokens>(
    atom: AstParser!(Expr),
    expr: AstParser!(Expr),
    lambda: AstParser!(Expr),
) -> AstParser!(Call) {
    let lambda_only = atom
        .clone()
        .map(Box::new)
        .map_with(|ex, e| (ex, e.span()))
        .then(lambda.clone().map_with(|ex, e| (ex, e.span())).clone())
        .map(|(name, lambda)| {
            let args = vec![lambda];
            Call { name, args }
        });
    let args = expr
        .map_with(|ex, e| (ex, e.span()))
        .separated_by(just(punct(',')).padded_by(optional_newline()))
        .allow_trailing()
        .collect::<Vec<_>>()
        .delimited_by(
            just(punct('(')).then(optional_newline()),
            optional_newline().then(just(punct(')'))),
        )
        .then(lambda.map_with(|ex, e| (ex, e.span())).or_not());

    let call = atom
        .map(Box::new)
        .map_with(|ex, e| (ex, e.span()))
        .then(args)
        .map(|(name, (mut args, lambda))| {
            if let Some(lambda) = lambda {
                args.push(lambda);
            }
            Call { name, args }
        });
    lambda_only.or(call)
}
