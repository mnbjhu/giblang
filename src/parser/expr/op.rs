use std::fmt::Display;

use crate::util::Spanned;

use super::Expr;

#[derive(Clone, PartialEq, Debug, Eq)]
pub struct Op {
    pub left: Box<Spanned<Expr>>,
    pub right: Box<Spanned<Expr>>,
    pub kind: OpKind,
}

#[derive(Clone, PartialEq, Debug, Eq)]
pub enum OpKind {
    Add,
    Sub,
    Mul,
    Div,
}

impl Display for OpKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpKind::Add => write!(f, "+"),
            OpKind::Sub => write!(f, "-"),
            OpKind::Mul => write!(f, "*"),
            OpKind::Div => write!(f, "/"),
        }
    }
}
