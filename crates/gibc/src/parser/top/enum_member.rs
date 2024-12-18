use chumsky::Parser;

use crate::{parser::common::ident::spanned_ident_parser, util::Spanned, AstParser};

use super::struct_body::{struct_body_parser, StructBody};

#[derive(Debug, PartialEq, Clone)]
pub struct EnumMember {
    pub name: Spanned<String>,
    pub body: Spanned<StructBody>,
}

#[must_use]
pub fn enum_member_parser<'tokens, 'src: 'tokens>() -> AstParser!(EnumMember) {
    spanned_ident_parser()
        .then(struct_body_parser().map_with(|b, e| (b, e.span())))
        .map(|(name, body)| EnumMember { name, body })
}
