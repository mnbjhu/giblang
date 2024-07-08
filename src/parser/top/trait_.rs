use chumsky::{primitive::just, IterParser, Parser};

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
    },
    util::Spanned,
    AstParser,
};

use super::func::{func_parser, Func};

#[derive(Clone)]
pub struct Trait {
    pub name: Spanned<String>,
    pub generics: GenericArgs,
    pub body: Vec<Spanned<Func>>,
}

pub fn trait_parser<'tokens, 'src: 'tokens>(stmt: AstParser!(Stmt)) -> AstParser!(Trait) {
    let body = func_parser(stmt)
        .map_with(|s, e| (s, e.span()))
        .separated_by(just(newline()))
        .collect()
        .delimited_by(
            just(punct('{')).then(optional_newline()),
            optional_newline().then(just(punct('}'))),
        );
    just(kw!(trait))
        .ignore_then(spanned_ident_parser())
        .then(generic_args_parser())
        .then(body)
        .map(|((name, generics), body)| Trait {
            name,
            generics,
            body,
        })
}
