use crate::util::Spanned;
use super::Expr;

#[derive(Clone, PartialEq, Debug, Eq)]
pub struct Field {
    pub name: Spanned<String>,
    pub struct_: Box<Spanned<Expr>>,
}
