use chumsky::Parser;

use crate::{parser::common::ident::spanned_ident_parser, util::Spanned, AstParser};

use super::struct_body::{struct_body_parser, StructBody};

#[derive(Debug, PartialEq, Clone)]
pub struct EnumMember {
    pub id: u32,
    pub name: Spanned<String>,
    pub body: StructBody,
}

pub fn enum_member_parser<'tokens, 'src: 'tokens>() -> AstParser!(EnumMember) {
    spanned_ident_parser()
        .then(struct_body_parser())
        .map_with(|(name, body), e| {
            let state: &mut u32 = e.state();
            *state += 1;
            EnumMember {
                name,
                body,
                id: *state,
            }
        })
}
