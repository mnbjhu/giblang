use crate::{
    check::state::{CheckState, FoundItem},
    parser::expr::Expr,
};

use super::ty::type_name;

mod call;
mod lit;
mod match_;

impl Expr {
    pub fn build(&self, state: &mut CheckState, kind: &ExprKind) -> String {
        match self {
            Expr::Literal(lit) => lit.build(kind),
            Expr::Ident(ident) => match state.get_name(ident) {
                FoundItem::Var(_) => ident[0].0.to_string(),
                FoundItem::Decl(name) => {
                    println!(
                        "{}",
                        ident
                            .iter()
                            .map(|x| x.0.to_string())
                            .collect::<Vec<_>>()
                            .join(".")
                    );
                    type_name(name)
                }
            },
            Expr::Call(call) => call.build(state, kind),
            Expr::Match(_) => todo!(),
            Expr::MemberCall(_) => todo!(),
            Expr::Tuple(_) => todo!(),
            Expr::IfElse(_) => todo!(),
            Expr::CodeBlock(_) => todo!(),
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
