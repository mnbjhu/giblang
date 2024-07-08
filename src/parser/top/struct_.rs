use crate::parser::common::optional_newline::optional_newline;
use crate::AstParser;
use crate::{
    lexer::token::punct,
    parser::common::{
        generic_args::{generic_args_parser, GenericArgs},
        ident::spanned_ident_parser,
    },
    util::Spanned,
};
use chumsky::IterParser;
use chumsky::{primitive::just, Parser};

use super::struct_field::{struct_field_parser, StructField};
#[derive(Debug, PartialEq, Clone)]
pub struct Struct {
    pub name: Spanned<String>,
    pub generics: Spanned<GenericArgs>,
    pub fields: Vec<Spanned<StructField>>,
}

pub fn struct_parser<'tokens, 'src: 'tokens>() -> AstParser!(Struct) {
    let name = spanned_ident_parser();
    let generics = generic_args_parser().map_with(|t, s| (t, s.span()));
    let fields = struct_field_parser()
        .map_with(|t, s| (t, s.span()))
        .separated_by(just(punct(',')).padded_by(optional_newline()))
        .collect::<Vec<_>>()
        .delimited_by(
            just(punct('{')).then(optional_newline()),
            optional_newline().then(just(punct('}'))),
        );
    name.then(generics)
        .then(fields)
        .map(|((name, generics), fields)| Struct {
            name,
            generics,
            fields,
        })
}
