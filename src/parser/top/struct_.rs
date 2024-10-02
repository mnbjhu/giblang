use crate::{kw, AstParser};
use crate::{
    parser::common::{
        generic_args::{generic_args_parser, GenericArgs},
        ident::spanned_ident_parser,
    },
    util::Spanned,
};
use chumsky::{primitive::just, Parser};

use super::struct_body::{struct_body_parser, StructBody};

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Struct {
    pub name: Spanned<String>,
    pub generics: Spanned<GenericArgs>,
    pub body: StructBody,
}

#[must_use]
pub fn struct_parser<'tokens, 'src: 'tokens>() -> AstParser!(Struct) {
    let name = spanned_ident_parser();
    let generics = generic_args_parser().map_with(|t, s| (t, s.span()));
    just(kw!(struct))
        .ignore_then(name)
        .then(generics)
        .then(struct_body_parser())
        .map(|((name, generics), body)| Struct {
            name,
            generics,
            body,
        })
}
