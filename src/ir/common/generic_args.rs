use crate::{
    check::{state::CheckState, SemanticToken},
    ir::{ContainsOffset, IrNode, IrState},
    parser::common::generic_args::GenericArgs,
    util::Spanned,
};

use super::generic_arg::GenericArgIR;

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct GenericArgsIR<'db>(pub Vec<Spanned<GenericArgIR<'db>>>);

impl<'db> GenericArgs {
    pub fn check(&self, state: &mut CheckState<'db>) -> GenericArgsIR<'db> {
        GenericArgsIR(
            self.0
                .iter()
                .map(|(arg, span)| (arg.check(state), *span))
                .collect(),
        )
    }
}

impl<'db> IrNode<'db> for GenericArgsIR<'db> {
    fn tokens(&self, tokens: &mut Vec<SemanticToken>, state: &mut IrState<'db>) {
        for arg in &self.0 {
            arg.0.tokens(tokens, state);
        }
    }

    fn at_offset(&self, offset: usize, state: &mut IrState<'db>) -> &dyn IrNode {
        for (arg, span) in &self.0 {
            if span.contains_offset(offset) {
                return arg.at_offset(offset, state);
            }
        }
        self
    }
}
