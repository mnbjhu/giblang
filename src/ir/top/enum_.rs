use crate::{
    check::{state::CheckState, SemanticToken, TokenKind},
    ir::{common::generic_args::GenericArgsIR, IrNode},
    item::common::type_::ContainsOffset,
    parser::top::enum_::Enum,
    util::Spanned,
};

use super::enum_member::EnumMemberIR;

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct EnumIR<'db> {
    pub name: Spanned<String>,
    pub generics: Spanned<GenericArgsIR<'db>>,
    pub members: Vec<Spanned<EnumMemberIR<'db>>>,
}
impl<'db> Enum {
    pub fn check(&self, state: &mut CheckState<'db>) -> EnumIR<'db> {
        state.path.push(self.name.0.to_string());
        let generics = (self.generics.0.check(state), self.generics.1);
        let members = self
            .members
            .iter()
            .map(|(member, span)| (member.check(state), *span))
            .collect();

        state.path.pop();
        EnumIR {
            name: self.name.clone(),
            generics,
            members,
        }
    }
}

impl<'db> IrNode<'db> for EnumIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut crate::ir::IrState<'db>) -> &dyn IrNode {
        if self.generics.1.contains_offset(offset) {
            return self.generics.0.at_offset(offset, state);
        }
        for (member, span) in &self.members {
            if span.contains_offset(offset) {
                return member.at_offset(offset, state);
            }
        }
        self
    }

    fn tokens(
        &self,
        tokens: &mut Vec<crate::check::SemanticToken>,
        state: &mut crate::ir::IrState<'db>,
    ) {
        tokens.push(SemanticToken {
            span: self.name.1,
            kind: TokenKind::Enum,
        })
    }
}
