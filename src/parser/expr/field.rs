use chumsky::{primitive::just, Parser};

use crate::{
    lexer::token::punct, parser::common::{ident::spanned_ident_parser, optional_newline::optional_newline}, util::Spanned, AstParser,
};

use super::Expr;

#[derive(Clone, PartialEq, Debug, Eq)]
pub struct Field {
    pub name: Spanned<String>,
    pub struct_: Box<Spanned<Expr>>,
}

pub fn field_parser<'tokens, 'src: 'tokens>(atom: AstParser!(Expr)) -> AstParser!(Field) {
    let struct_ = atom.map_with(|ex, e| Box::new((ex, e.span())));
    let name = spanned_ident_parser();
    struct_

        .then_ignore(just(punct('.')).padded_by(optional_newline()))
        .then(name)
        .map(|(struct_, name)| Field { name, struct_ })
}
