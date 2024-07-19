use chumsky::{primitive::just, IterParser, Parser};

use crate::{
    kw,
    lexer::token::{newline, punct},
    parser::{
        common::{
            generic_args::{generic_args_parser, GenericArgs},
            optional_newline::optional_newline,
            type_::{named_parser, type_parser, NamedType},
        },
        stmt::Stmt,
    },
    util::Spanned,
    AstParser,
};

use super::func::{func_parser, Func};

#[derive(Debug, PartialEq, Clone)]
pub struct Impl {
    pub generics: GenericArgs,
    pub trait_: Spanned<NamedType>,
    pub for_: Spanned<NamedType>,
    pub body: Vec<Spanned<Func>>,
}

pub fn impl_parser<'tokens, 'src: 'tokens>(stmt: AstParser!(Stmt)) -> AstParser!(Impl) {
    let trait_ = named_parser(type_parser())
        .map_with(|t, e| (t, e.span()))
        .then_ignore(just(kw!(for)));
    let for_ = named_parser(type_parser()).map_with(|t, e| (t, e.span()));

    let body = func_parser(stmt)
        .map_with(|s, e| (s, e.span()))
        .separated_by(just(newline()))
        .collect()
        .delimited_by(
            just(punct('{')).then(optional_newline()),
            optional_newline().then(just(punct('}'))),
        )
        .or_not()
        .map(|body| body.unwrap_or_default());

    just(kw!(impl))
        .ignore_then(generic_args_parser())
        .then(trait_)
        .then(for_)
        .then(body)
        .map(|(((generics, trait_), for_), body)| Impl {
            generics,
            trait_,
            for_,
            body,
        })
}
