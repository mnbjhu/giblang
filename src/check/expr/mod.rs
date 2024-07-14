use crate::{fs::project::Project, lexer::literal::Literal, parser::expr::Expr};

use super::{
    ty::{PrimTy, Ty},
    CheckState, NamedExpr,
};

impl Expr {
    pub fn check<'module>(
        &'module self,
        project: &'module Project,
        state: &mut CheckState<'module>,
    ) -> Ty<'module> {
        match self {
            Expr::Literal(lit) => lit.into(),
            Expr::Ident(ident) => {
                let expr = state.get_path(ident, project, true);
                expr.into()
            }
            Expr::CodeBlock(block) => {
                state.enter_scope();
                for (stmt, _) in block {
                    stmt.check(project, state);
                }
                state.exit_scope();

                // TODO: Add block return types
                Ty::Unknown
            }
        }
    }
}

impl<'module> From<&Literal> for Ty<'module> {
    fn from(value: &Literal) -> Self {
        match value {
            Literal::Int(_) => Ty::Prim(PrimTy::Int),
            Literal::Float(_) => Ty::Prim(PrimTy::Float),
            Literal::String(_) => Ty::Prim(PrimTy::String),
            Literal::Bool(_) => Ty::Prim(PrimTy::Bool),
            Literal::Char(_) => Ty::Prim(PrimTy::Char),
        }
    }
}

impl<'module> From<NamedExpr<'module>> for Ty<'module> {
    fn from(value: NamedExpr<'module>) -> Self {
        match value {
            NamedExpr::Imported(export, _) => Ty::Named {
                name: export.clone(),
                args: vec![],
            },
            NamedExpr::Variable(ty) => ty.clone(),
            NamedExpr::GenericArg {
                name,
                super_,
                variance,
            } => Ty::Generic {
                name: name.to_string(),
                variance,
                super_: Box::new(super_.clone()),
            },
            NamedExpr::Prim(p) => Ty::Prim(p.clone()),
            NamedExpr::Unknown => Ty::Unknown,
        }
    }
}
