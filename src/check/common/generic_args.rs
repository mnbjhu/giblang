use std::ops::ControlFlow;

use crate::{check::{state::CheckState, Check, ControlIter}, item::AstItem, parser::common::generic_args::GenericArgs, ty::Ty, util::Span};

impl<'ast, 'db, Iter: ControlIter<'ast, 'db>> Check<'ast, 'db, Iter, Vec<Ty<'db>>> for GenericArgs {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        _: Span,
        (): (),
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), Vec<Ty<'db>>> {
        let mut args = vec![];
        for (arg, span) in &self.0 {
            args.push(arg.check(state, control, *span, ())?);
        }
        ControlFlow::Continue(args)
    }
}
