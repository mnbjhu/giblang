use chumsky::primitive::just;
use chumsky::recovery::via_parser;
use chumsky::Parser;

use crate::lexer::token::punct;
use crate::parser::common::ident::spanned_ident_parser;
use crate::parser::common::type_::type_parser;
use crate::util::Span;
use crate::AstParser;
use crate::{parser::common::type_::Type, util::Spanned};

#[derive(Debug, PartialEq, Clone)]
pub struct StructField {
    pub name: Spanned<String>,
    pub ty: Spanned<Type>,
}

#[must_use]
pub fn struct_field_parser<'tokens, 'src: 'tokens>() -> AstParser!(StructField) {
    let missing_type = spanned_ident_parser()
        .then_ignore(just(punct(':')).or_not())
        .map(|name| {
            let span = Span::splat(name.1.end);
            StructField {
                name,
                ty: (Type::Error(span), span),
            }
        });
    spanned_ident_parser()
        .then_ignore(just(punct(':')))
        .then(type_parser().map_with(|t, s| (t, s.span())))
        .map(|(name, ty)| StructField { name, ty })
        .recover_with(via_parser(missing_type))
}
