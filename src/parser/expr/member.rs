use crate::util::Spanned;

use super::Expr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemberCall {
    pub rec: Box<Spanned<Expr>>,
    pub name: Spanned<String>,
    pub args: Vec<Spanned<Expr>>,
}
