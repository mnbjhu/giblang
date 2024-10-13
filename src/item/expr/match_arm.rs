use crate::{
    item::{common::type_::ContainsOffset as _, AstItem},
    parser::expr::match_arm::MatchArm,
};

impl AstItem for MatchArm {
    fn at_offset<'me>(
        &'me self,
        state: &mut crate::check::state::CheckState,
        offset: usize,
    ) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        if self.pattern.1.contains_offset(offset) {
            return self.pattern.0.at_offset(state, offset);
        }
        if self.expr.1.contains_offset(offset) {
            return self.expr.0.at_offset(state, offset);
        }
        self
    }

    fn tokens(
        &self,
        state: &mut crate::check::state::CheckState,
        tokens: &mut Vec<crate::check::SemanticToken>,
    ) {
        self.pattern.0.tokens(state, tokens);
        self.expr.0.tokens(state, tokens);
    }
}
