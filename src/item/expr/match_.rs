use crate::{
    item::{common::type_::ContainsOffset as _, AstItem},
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
        for branch in &self.arms {
            if branch.1.contains_offset(offset) {
                return branch.0.at_offset(state, offset);
            }
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
}