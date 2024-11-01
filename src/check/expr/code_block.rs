use std::ops::ControlFlow;

use crate::{
    check::{state::CheckState, Check, ControlIter},
    item::AstItem,
    parser::expr::code_block::CodeBlock,
    ty::Ty,
    util::Span,
};

impl<'ast, 'db, Iter: ControlIter<'ast>> Check<'ast, 'db, Iter, Ty<'db>> for CodeBlock {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: Span,
        (): (),
    ) -> ControlFlow<&'ast dyn AstItem, Ty<'db>> {
        state.enter_scope();
        let mut ret = Ty::unit();
        for stmt in self {
            ret = stmt.check(state, control, (), span)?;
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
        args: (),
    ) -> ControlFlow<&'ast dyn AstItem, Ty<'db>> {
        if self.is_empty() {
            Ty::unit().expect_is_instance_of(expected, state, false, span);
            return;
        }
        state.enter_scope();
        for stmt in &self[0..self.len() - 1] {
            stmt.check(state, control, expected, stmt.1);
        }
        let last = self.last().unwrap();
        last.expect(state, control, expected, last.1, ());
        state.exit_scope();
    }
}
