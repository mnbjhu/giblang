use std::ops::ControlFlow;

use crate::{
    check::{state::CheckState, Check, ControlIter, Dir},
    item::AstItem,
    parser::expr::code_block::CodeBlock,
    ty::Ty,
    util::Span,
};

impl<'ast, 'db, Iter: ControlIter<'ast, 'db>> Check<'ast, 'db, Iter> for CodeBlock {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: Span,
        (): (),
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), Ty<'db>> {
        state.enter_scope();
        control.act(self, state, Dir::Enter, span)?;
        let mut ret = Ty::unit();
        for stmt in self {
            ret = stmt.0.check(state, control, stmt.1, ())?;
        }
        state.exit_scope();
        ControlFlow::Continue(ret)
    }

    fn expect(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        expected: &Ty<'db>,
        span: Span,
        (): (),
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), Ty<'db>> {
        if expected.is_unit() {
            return self.check(state, control, span, ());
        }
        if self.is_empty() {
            Ty::unit().expect_is_instance_of(expected, state, false, span);
            return ControlFlow::Continue(Ty::unit());
        }
        state.enter_scope();
        for stmt in &self[0..self.len() - 1] {
            stmt.0.check(state, control, stmt.1, ())?;
        }
        let last = self.last().unwrap();
        let ty = last.0.expect(state, control, expected, last.1, ())?;
        state.exit_scope();
        ControlFlow::Continue(ty)
    }
}
