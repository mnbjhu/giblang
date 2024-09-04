use crate::{
    check::state::{CheckState, FoundItem},
    parser::expr::Expr,
};

use super::ty::type_name;

mod call;
mod lit;
mod match_;
mod member;
mod property;

impl Expr {
    pub fn build(&self, state: &mut CheckState, kind: &ExprKind) -> String {
        match self {
            Expr::Literal(lit) => lit.build(kind),
            Expr::Ident(ident) => {
                let name = match state.get_name(ident) {
                    FoundItem::Var(_) => ident[0].0.to_string(),
                    FoundItem::Decl(name) => type_name(name),
                };
                kind.basic_apply(name)
            }
            Expr::Call(call) => call.build(state, kind),
            Expr::Match(_) => todo!(),
            Expr::MemberCall(member) => member.build(state, kind),
            Expr::Tuple(_) => todo!(),
            Expr::IfElse(_) => todo!(),
            Expr::CodeBlock(_) => todo!(),
            Expr::Property(prop) => prop.build(state, kind),
        }
    }
}

pub enum ExprKind {
    Inline,
    Return,
    Assign(String),
}

impl ExprKind {
    #[must_use]
    pub fn basic_apply(&self, expr: String) -> String {
        match &self {
            ExprKind::Inline => expr,
            ExprKind::Return => format!("return {expr}"),
            ExprKind::Assign(name) => format!("{name} = {expr}"),
        }
    }
}
