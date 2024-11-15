use crate::{
    check::state::CheckState,
    ir::{common::pattern::PatternIR, expr::ExprIR, ty::TypeIR, ContainsOffset, IrNode},
    parser::stmt::let_::LetStatement,
    util::Spanned,
};
#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Test {
    text: Spanned<String>,
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct LetIR<'db> {
    pub pattern: Spanned<PatternIR<'db>>,
    pub ty: Option<Spanned<TypeIR<'db>>>,
    pub expr: Box<Spanned<ExprIR<'db>>>,
}
impl<'db> LetStatement {
    pub fn check(&self, state: &mut CheckState<'db>) -> LetIR<'db> {
        let mut ty = None;
        let expr = if let Some(expected) = &self.ty {
            let expected = (expected.0.check(state), expected.1);
            let expr = self.value.0.expect(state, &expected.0.ty, self.value.1);
            ty = Some(expected);
            (expr, self.value.1)
        } else {
            (self.value.0.check(state), self.value.1)
        };
        let pattern = (self.pattern.0.expect(state, &expr.0.ty), self.pattern.1);
        LetIR {
            pattern,
            expr: Box::new(expr),
            ty,
        }
    }
}

impl<'db> IrNode<'db> for LetIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut crate::ir::IrState<'db>) -> &dyn IrNode {
        if self.pattern.1.contains_offset(offset) {
            return self.pattern.0.at_offset(offset, state);
        }
        if let Some(ty) = &self.ty {
            if ty.1.contains_offset(offset) {
                return ty.0.at_offset(offset, state);
            }
        }
        if self.expr.1.contains_offset(offset) {
            return self.expr.0.at_offset(offset, state);
        }
        self
    }

    fn tokens(
        &self,
        tokens: &mut Vec<crate::check::SemanticToken>,
        state: &mut crate::ir::IrState<'db>,
    ) {
        self.pattern.0.tokens(tokens, state);
        if let Some(ty) = &self.ty {
            ty.0.tokens(tokens, state);
        }
        self.expr.0.tokens(tokens, state);
    }
}
