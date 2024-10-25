use chumsky::{primitive::just, recovery::via_parser, IterParser as _, Parser as _};

use crate::{
    lexer::token::{newline, punct},
    op,
    parser::{
        common::{
            optional_newline::optional_newline,
            pattern::{pattern_parser, Pattern},
            type_::{type_parser, Type},
        },
        stmt::Stmt,
        top_recovery,
    },
    util::Spanned,
    AstParser,
};

use super::code_block::CodeBlock;

#[derive(Clone, PartialEq, Debug, Eq)]
pub struct Lambda {
    pub args: Vec<Spanned<LambdaParam>>,
    pub body: Spanned<CodeBlock>,
}

#[derive(Clone, PartialEq, Debug, Eq)]
pub struct LambdaParam {
    pub pattern: Spanned<Pattern>,
    pub ty: Option<Spanned<Type>>,
}

pub fn lambda_parser<'tokens, 'src: 'tokens>(stmt: AstParser!(Stmt)) -> AstParser!(Lambda) {
    let stmts = stmt
        .map_with(|s, e| Some((s, e.span())))
        .recover_with(via_parser(top_recovery().map(|()| None)))
        .separated_by(just(newline()))
        .collect()
        .map_with(|stmts: Vec<_>, e| (stmts.into_iter().flatten().collect(), e.span()));

    let args = lambda_param_parser()
        .map_with(|p, e| (p, e.span()))
        .separated_by(just(punct(',')).padded_by(optional_newline()))
        .collect::<Vec<_>>()
        .then_ignore(just(op!(->)).padded_by(optional_newline()));

    args.then(stmts)
        .delimited_by(
            just(punct('{')).then(optional_newline()),
            optional_newline().then(just(punct('}'))),
        )
        .map(|(args, body)| Lambda { args, body })
}

pub fn lambda_param_parser<'tokens, 'src: 'tokens>() -> AstParser!(LambdaParam) {
    let ty = just(op!(':')).ignore_then(type_parser().map_with(|t, e| (t, e.span())));
    pattern_parser()
        .map_with(|p, e| (p, e.span()))
        .then(ty.or_not())
        .map(|(pattern, ty)| LambdaParam { pattern, ty })
}
