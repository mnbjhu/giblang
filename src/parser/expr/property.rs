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
pub fn property_parser<'tokens, 'src: 'tokens>(atom: AstParser!(Expr)) -> AstParser!(Property) {
    atom.map(Box::new)
        .map_with(|a, e| (a, e.span()))
        .then_ignore(just(punct('.')))
        .then(spanned_ident_parser())
        .map(|(expr, name)| Property { expr, name })
}
