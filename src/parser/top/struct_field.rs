use chumsky::primitive::just;
use chumsky::Parser;

use crate::lexer::token::punct;
use crate::parser::common::ident::spanned_ident_parser;
use crate::parser::common::type_::type_parser;
use crate::AstParser;
use crate::{parser::common::type_::Type, util::Spanned};

#[derive(Debug, PartialEq, Clone)]
pub struct StructField {
    pub name: Spanned<String>,
    pub ty: Spanned<Type>,
}

pub fn struct_field_parser<'tokens, 'src: 'tokens>() -> AstParser!(StructField) {
    spanned_ident_parser()
        .then_ignore(just(punct(':')))
        .then(type_parser().map_with(|t, s| (t, s.span())))
        .map(|(name, ty)| StructField { name, ty })
}
