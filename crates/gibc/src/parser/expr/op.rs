use std::fmt::Display;

use chumsky::{select, Parser as _};

use crate::{lexer::token::Token, util::Spanned, AstParser};

use super::Expr;

#[derive(Clone, PartialEq, Debug)]
pub struct Op {
    pub left: Box<Spanned<Expr>>,
    pub right: Box<Spanned<Expr>>,
    pub kind: OpKind,
}

#[derive(Clone, PartialEq, Debug)]
pub enum OpKind {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
    And,
    Or,
}

impl Display for OpKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpKind::Add => write!(f, "+"),
            OpKind::Sub => write!(f, "-"),
            OpKind::Mul => write!(f, "*"),
            OpKind::Div => write!(f, "/"),
            OpKind::Mod => write!(f, "%"),
            OpKind::Lt => write!(f, "<"),
            OpKind::Gt => write!(f, ">"),
            OpKind::Lte => write!(f, "<="),
            OpKind::Gte => write!(f, ">="),
            OpKind::Eq => write!(f, "=="),
            OpKind::Neq => write!(f, "!="),
            OpKind::And => write!(f, "&&"),
            OpKind::Or => write!(f, "||"),
        }
    }
}

pub fn op_parser<'tokens, 'src: 'tokens>(expr: AstParser!(Expr)) -> AstParser!(Expr) {
    let mul_op = select! {
        Token::Op(op) if op == "*" => OpKind::Mul,
        Token::Op(op) if op == "/" => OpKind::Div,
        Token::Op(op) if op == "%" => OpKind::Mod,
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
        .map(|(a, _)| a)
        .boxed();

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
        .map(|(a, _)| a)
        .boxed();

    let eq_op = select! {
        Token::Op(op) if op == "==" => OpKind::Eq,
        Token::Op(op) if op == "!=" => OpKind::Neq,
    };

    let eq = sum
        .clone()
        .map_with(|a, e| (a, e.span()))
        .foldl_with(
            eq_op
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
        .boxed();

    let cmp_op = select! {
        Token::Op(op) if op == "<" => OpKind::Lt,
        Token::Op(op) if op == ">" => OpKind::Gt,
        Token::Op(op) if op == "<=" => OpKind::Lte,
        Token::Op(op) if op == ">=" => OpKind::Gte,
    };

    let cmp = eq
        .clone()
        .map_with(|a, e| (a, e.span()))
        .foldl_with(
            cmp_op
                .then(eq.clone().map_with(|a, e| (a, e.span())))
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
        .boxed();

    let and = cmp
        .clone()
        .map_with(|a, e| (a, e.span()))
        .foldl_with(
            select! {
                Token::Op(op) if op == "&&" => OpKind::And,
            }
            .then(cmp.clone().map_with(|a, e| (a, e.span())))
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
        .boxed();

    let or = and
        .clone()
        .map_with(|a, e| (a, e.span()))
        .foldl_with(
            select! {
                Token::Op(op) if op == "||" => OpKind::Or,
            }
            .then(and.clone().map_with(|a, e| (a, e.span())))
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
        .boxed();
    or
}
