use gvm::format::instr::ByteCode;

use crate::{
    check::{build_state::BuildState, state::CheckState},
    ir::{common::pattern::PatternIR, expr::ExprIR, ty::TypeIR, ContainsOffset, IrNode},
    parser::stmt::let_::LetStatement,
    ty::Ty,
    util::Spanned,
};
#[derive(Debug, PartialEq, Clone)]
pub struct Test {
    text: Spanned<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LetIR<'db> {
    pub pattern: Spanned<PatternIR<'db>>,
    pub ty: Option<Spanned<TypeIR<'db>>>,
    pub expr: Box<Spanned<ExprIR<'db>>>,
}
impl<'db> LetStatement {
    pub fn check(&self, state: &mut CheckState<'db>) -> LetIR<'db> {
        let explicit = self.ty.as_ref().map(|(ty, span)| (ty.check(state), *span));
        let expr = if let Some(explicit) = &explicit {
            let expr = self.value.0.expect(state, &explicit.0.ty, self.value.1);
            expr
        } else {
            let expr = self.value.0.check(state);
            expr
        };
        let pattern = if let Some(explicit) = &explicit {
            (self.pattern.0.expect(state, &explicit.0.ty), self.pattern.1)
        } else if Ty::Unknown == expr.ty {
            (self.pattern.0.check(state), self.pattern.1)
        } else {
            (self.pattern.0.expect(state, &expr.ty), self.pattern.1)
        };
        LetIR {
            pattern,
            expr: Box::new((expr, self.value.1)),
            ty: explicit,
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

impl<'db> LetIR<'db> {
    pub fn build(&self, state: &mut BuildState<'db>) -> Vec<ByteCode> {
        let mut code = vec![];
        code.extend(self.expr.0.build(state));
        code.extend(self.pattern.0.build(state));
        code
    }
}
