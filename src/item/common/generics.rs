use crate::{check::state::CheckState, item::AstItem, parser::common::generic_args::GenericArgs};

impl AstItem for GenericArgs {
    fn at_offset<'me>(&'me self, state: &mut CheckState, offset: usize) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        for (arg, span) in &self.0 {
            arg.check(state);
            if span.start <= offset && offset <= span.end {
                return arg.at_offset(state, offset);
            }
        }
        self
    }

    fn tokens(&self, state: &mut CheckState, tokens: &mut Vec<crate::check::SemanticToken>) {
        for (arg, _) in &self.0 {
            arg.tokens(state, tokens);
        }
    }
}
