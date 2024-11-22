use chumsky::{primitive::just, recovery::via_parser, IterParser, Parser};

use crate::{
    kw,
    lexer::token::{newline, punct},
    parser::{
        common::{
            generic_args::{generic_args_parser, GenericArgs},
            ident::spanned_ident_parser,
            optional_newline::optional_newline,
        },
        stmt::Stmt,
        top_recovery,
    },
    util::Spanned,
    AstParser,
};

use super::func::{func_parser, Func};

#[derive(Debug, PartialEq, Clone)]
pub struct Trait {
    pub name: Spanned<String>,
    pub generics: Spanned<GenericArgs>,
    pub body: Vec<Spanned<Func>>,
}

pub fn trait_parser<'tokens, 'src: 'tokens>(stmt: AstParser!(Stmt)) -> AstParser!(Trait) {
    let body = func_parser(stmt)
        .map_with(|s, e| (s, e.span()))
        .map(Option::Some)
        .recover_with(via_parser(top_recovery().map(|()| None)))
        .separated_by(just(newline()))
        .collect::<Vec<_>>()
        .map(|v| v.into_iter().flatten().collect())
        .delimited_by(
            just(punct('{')).then(optional_newline()),
            optional_newline().then(just(punct('}'))),
        )
        .or_not()
        .map(std::option::Option::unwrap_or_default);
    just(kw!(trait))
        .ignore_then(spanned_ident_parser())
        .then(generic_args_parser().map_with(|g, e| (g, e.span())))
        .then(body)
        .map(|((name, generics), body)| Trait {
            name,
            generics,
            body,
        })
}
