use crate::{
    item::{
        common::{generics::brackets_big, type_::ContainsOffset as _},
        AstItem,
    },
    parser::expr::match_::Match,
};

impl AstItem for Match {
    fn at_offset<'me>(
        &'me self,
        state: &mut crate::check::state::CheckState,
        offset: usize,
    ) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        if self.expr.1.contains_offset(offset) {
            return self.expr.0.at_offset(state, offset);
        }
        let ty = self.expr.0.check(state);
        for branch in &self.arms {
            state.enter_scope();
            branch.0.pattern.0.check(state, &ty.clone());
            if branch.1.contains_offset(offset) {
                return branch.0.at_offset(state, offset);
            }
            state.exit_scope();
        }
        self
    }

    fn tokens(
        &self,
        state: &mut crate::check::state::CheckState,
        tokens: &mut Vec<crate::check::SemanticToken>,
    ) {
        self.expr.0.tokens(state, tokens);
        for arm in &self.arms {
            arm.0.tokens(state, tokens);
        }
    }

    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        allocator
            .text("match")
            .append(allocator.space())
            .append(self.expr.0.pretty(allocator))
            .append(allocator.space())
            .append(brackets_big(allocator, "{", "}", &self.arms))
    }
}
