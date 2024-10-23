use std::fmt::Display;

use chumsky::{select, Parser as _};

use crate::{lexer::token::Token, util::Spanned, AstParser};

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
    Eq,
    Neq,
}

impl Display for OpKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpKind::Add => write!(f, "+"),
            OpKind::Sub => write!(f, "-"),
            OpKind::Mul => write!(f, "*"),
            OpKind::Div => write!(f, "/"),
            OpKind::Eq => write!(f, "=="),
            OpKind::Neq => write!(f, "!="),
        }
    }
}

pub fn op_parser<'tokens, 'src: 'tokens>(expr: AstParser!(Expr)) -> AstParser!(Expr) {

        let mul_op = select! {
            Token::Op(op) if op == "*" => OpKind::Mul,
            Token::Op(op) if op == "/" => OpKind::Div,
        };

        let mul = expr
            .clone()
            .map_with(|a, e| (a, e.span()))
            .foldl_with(
                mul_op
                    .then(expr.clone().map_with(|a, e| (a, e.span())))
                    .repeated(),
                |a, (kind, b), e| {
                    (
                        Expr::Op(Op {
                            left: Box::new(a),
                            kind,
                            right: Box::new(b),
                        }),
                        e.span(),
                    )
                },
            )
            .map(|(a, _)| a);


        let sum_op = select! {
            Token::Op(op) if op == "+" => OpKind::Add,
            Token::Op(op) if op == "-" => OpKind::Sub,
        };

        let sum = mul
            .clone()
            .map_with(|a, e| (a, e.span()))
            .foldl_with(
                sum_op
                    .then(mul.clone().map_with(|a, e| (a, e.span())))
                    .repeated(),
                |a, (kind, b), e| {
                    (
                        Expr::Op(Op {
                            left: Box::new(a),
                            kind,
                            right: Box::new(b),
                        }),
                        e.span(),
                    )
                },
            )
            .map(|(a, _)| a);

        let eq = select! {
            Token::Op(op) if op == "==" => OpKind::Eq,
            Token::Op(op) if op == "!=" => OpKind::Neq,
        };

        sum
            .clone()
            .map_with(|a, e| (a, e.span()))
            .foldl_with(
                eq
                    .then(sum.clone().map_with(|a, e| (a, e.span())))
                    .repeated(),
                |a, (kind, b), e| {
                    (
                        Expr::Op(Op {
                            left: Box::new(a),
                            kind,
                            right: Box::new(b),
                        }),
                        e.span(),
                    )
                },
            )
            .map(|(a, _)| a)
}
