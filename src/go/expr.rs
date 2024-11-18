use std::collections::HashMap;

use crate::{
    db::input::Db,
    item::{common::generics::brackets, AstItem},
    lexer::literal::Literal,
    parser::expr::op::OpKind,
};

use super::ty::GoType;

pub struct GoBuildState<'db> {
    pub scopes: Vec<HashMap<String, u32>>,
    pub db: &'db dyn Db,
}

#[derive(Debug)]
pub enum GoExpr {
    Ident(String),
    Literal(Literal),
    Call {
        receiver: Option<Box<GoExpr>>,
        name: String,
        args: Vec<GoExpr>,
    },
    Op {
        left: Box<GoExpr>,
        right: Box<GoExpr>,
        kind: OpKind,
    },
    Slice {
        ty: GoType,
        elements: Vec<GoExpr>,
    },
}

impl AstItem for GoExpr {
    fn item_name(&self) -> &'static str {
        "GoExpr"
    }

    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        match self {
            GoExpr::Ident(name) => allocator.text(name),
            GoExpr::Literal(lit) => lit.pretty(allocator),
            GoExpr::Call {
                receiver,
                name,
                args,
            } => {
                let receiver = match receiver {
                    Some(r) => r.pretty(allocator).append("."),
                    None => allocator.nil(),
                };
                receiver
                    .append(name)
                    .append(brackets(allocator, "(", ")", args))
            }
            GoExpr::Op { left, right, kind } => {
                let op = match kind {
                    OpKind::Add => "+",
                    OpKind::Sub => "-",
                    OpKind::Mul => "*",
                    OpKind::Div => "/",
                    OpKind::Eq => "==",
                    OpKind::Neq => "!=",
                    OpKind::Lt => "<",
                    OpKind::Gt => ">",
                    OpKind::Lte => "<=",
                    OpKind::Gte => ">=",
                    OpKind::And => "&&",
                    OpKind::Or => "||",
                };
                left.pretty(allocator)
                    .append(allocator.space())
                    .append(allocator.text(op))
                    .append(allocator.space())
                    .append(right.pretty(allocator))
            }
            GoExpr::Slice { ty, elements } => allocator
                .text("[]")
                .append(ty.pretty(allocator))
                .append(brackets(allocator, "{", "}", elements)),
        }
    }
}
