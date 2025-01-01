use std::fmt::Display;

use chumsky::{primitive::just, Parser as _};

use crate::{lexer::token::Token, util::Spanned, AstParser};

use super::Expr;

#[derive(Clone, PartialEq, Debug)]
pub struct UnaryOp {
    pub expr: Box<Spanned<Expr>>,
    pub kind: UnaryOpKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOpKind {
    Not,
}

impl Display for UnaryOpKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOpKind::Not => write!(f, "!"),
        }
    }
}

#[allow(clippy::too_many_lines)]
pub fn unary_op_parser<'tokens, 'src: 'tokens>(expr: AstParser!(Expr)) -> AstParser!(Expr) {
    just(Token::Op("!".to_string()))
        .ignore_then(expr.map_with(|ex, e| (ex, e.span())).map(Box::new))
        .map(|expr| {
            Expr::Unary(UnaryOp {
                expr,
                kind: UnaryOpKind::Not,
            })
        })
}
