use chumsky::{primitive::just, recovery::via_parser, IterParser, Parser};

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
        top_recovery,
    },
    util::Spanned,
    AstParser,
};

use super::func::{func_parser, Func};

#[derive(Debug, PartialEq, Clone)]
pub struct Impl {
    pub id: u32,
    pub generics: Spanned<GenericArgs>,
    pub trait_: Option<Spanned<NamedType>>,
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

    just(kw!(impl))
        .ignore_then(generic_args_parser().map_with(|g, e| (g, e.span())))
        .then(trait_.or_not())
        .then(for_)
        .then(body)
        .map_with(|(((generics, trait_), for_), body), e| {
            let counter: &mut u32 = e.state();
            *counter += 1;
            Impl {
                generics,
                trait_,
                for_,
                body,
                id: *counter,
            }
        })
}
