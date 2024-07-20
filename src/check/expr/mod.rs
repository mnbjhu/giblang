use crate::{
    fs::project::Project,
    parser::expr::Expr,
    ty::{Generic, Ty},
    util::Span,
};

use self::{
    code_block::{check_code_block, check_code_block_is},
    ident::{check_ident, check_ident_is},
    tuple::{check_tuple, check_tuple_is},
};

use super::{CheckState, NamedExpr};

pub mod call;
pub mod code_block;
pub mod ident;
pub mod lit;
pub mod match_;
pub mod match_arm;
pub mod tuple;

impl Expr {
    pub fn check<'module>(
        &'module self,
        project: &'module Project,
        state: &mut CheckState<'module>,
    ) -> Ty<'module> {
        match self {
            Expr::Literal(lit) => lit.into(),
            Expr::Ident(ident) => check_ident(state, ident, project),
            Expr::CodeBlock(block) => check_code_block(state, block, project),
            // TODO: Actually think about generics
            Expr::Call(call) => call.check(project, state),
            Expr::Match(match_) => match_.check(project, state),
            Expr::Tuple(values) => check_tuple(values, project, state),
            // TODO: Handle if else expr types
            Expr::IfElse(_) => todo!(),
        }
    }

    pub fn expect_instance_of<'module>(
        &'module self,
        expected: &Ty<'module>,
        project: &'module Project,
        state: &mut CheckState<'module>,
        span: Span,
    ) -> Ty<'module> {
        match self {
            Expr::Literal(lit) => lit.expect_instance_of(expected, project, state, span),
            Expr::Ident(ident) => check_ident_is(state, ident, expected, project),
            Expr::CodeBlock(block) => check_code_block_is(state, expected, block, project),
            Expr::Call(call) => call.expected_instance_of(expected, project, state, span),
            Expr::Match(match_) => match_.is_instance_of(expected, project, state),
            Expr::Tuple(v) => check_tuple_is(state, expected, v, project, span),
            Expr::IfElse(_) => todo!(),
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
            } => Ty::Generic(Generic {
                name: name.to_string(),
                variance,
                super_: Box::new(super_.clone()),
            }),
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
