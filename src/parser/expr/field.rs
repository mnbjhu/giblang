use super::Expr;
use crate::util::Spanned;

#[derive(Clone, PartialEq, Debug, Eq)]
pub struct Field {
    pub name: Spanned<String>,
    pub struct_: Box<Spanned<Expr>>,
}
