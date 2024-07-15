use crate::{fs::project::Project, parser::expr::Expr};

use self::ident::check_ident;

use super::{ty::Ty, CheckState, NamedExpr};

pub mod call;
pub mod ident;
pub mod lit;
pub mod match_;
pub mod match_arm;

impl Expr {
    pub fn check<'module>(
        &'module self,
        project: &'module Project,
        state: &mut CheckState<'module>,
    ) -> Ty<'module> {
        match self {
            Expr::Literal(lit) => lit.into(),
            Expr::Ident(ident) => check_ident(state, ident, project),
            Expr::CodeBlock(block) => {
                state.enter_scope();
                for (stmt, _) in block {
                    stmt.check(project, state);
                }
                state.exit_scope();

                // TODO: Add block return types
                Ty::Unknown
            }
            // TODO: Actually think about generics
            Expr::Call(call) => call.check(project, state),
            Expr::Match(match_) => match_.check(project, state),
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

#[cfg(test)]
mod tests {
    use crate::cli::build::build;

    #[test]
    fn test_crud() {
        build()
    }
}
