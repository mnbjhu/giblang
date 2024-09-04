use chumsky::{primitive::just, Parser};

use crate::{
    lexer::token::punct, parser::common::ident::spanned_ident_parser, util::Spanned, AstParser,
};

use super::Expr;

#[derive(Clone, PartialEq, Debug)]
pub struct Property {
    pub expr: Spanned<Box<Expr>>,
    pub name: Spanned<String>,
}
pub fn property_parser<'tokens, 'src: 'tokens>() -> AstParser!(PropertyAccess) {
    just(punct('.'))
        .ignore_then(spanned_ident_parser())
        .map(|name| PropertyAccess { name })
}

#[derive(Clone, PartialEq, Debug)]
pub struct PropertyAccess {
    pub name: Spanned<String>,
}
