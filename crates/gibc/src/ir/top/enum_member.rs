use crate::{
    check::{state::CheckState, SemanticToken, TokenKind},
    ir::{ContainsOffset, IrNode},
    parser::top::enum_member::EnumMember,
    util::Spanned,
};

use super::struct_body::StructBodyIR;

#[derive(Debug, PartialEq, Clone)]
pub struct EnumMemberIR<'db> {
    pub name: Spanned<String>,
    pub body: Spanned<StructBodyIR<'db>>,
}

impl<'db> EnumMember {
    pub fn check(&self, state: &mut CheckState<'db>) -> EnumMemberIR<'db> {
        let body = (self.body.0.check(state), self.body.1);
        EnumMemberIR {
            name: self.name.clone(),
            body,
        }
    }
}

impl<'db> IrNode<'db> for EnumMemberIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut crate::ir::IrState<'db>) -> &dyn IrNode {
        if self.body.1.contains_offset(offset) {
            self.body.0.at_offset(offset, state)
        } else {
            self
        }
    }

    fn tokens(&self, tokens: &mut Vec<SemanticToken>, state: &mut crate::ir::IrState<'db>) {
        tokens.push(SemanticToken {
            span: self.name.1,
            kind: TokenKind::Member,
        });
        self.body.0.tokens(tokens, state);
    }
}
