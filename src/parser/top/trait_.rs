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

#[derive(Debug, PartialEq, Clone)]
pub struct Trait {
    pub name: Spanned<String>,
    pub generics: GenericArgs,
    pub body: Vec<Spanned<Func>>,
    pub id: u32,
}

pub fn trait_parser<'tokens, 'src: 'tokens>(stmt: AstParser!(Stmt)) -> AstParser!(Trait) {
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
    just(kw!(trait))
        .ignore_then(spanned_ident_parser())
        .then(generic_args_parser())
        .then(body)
        .map_with(|((name, generics), body), e| {
            let state: &mut u32 = e.state();
            *state += 1;
            Trait {
                name,
                generics,
                body,
                id: *state,
            }
        })
}
