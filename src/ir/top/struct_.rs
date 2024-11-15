use crate::{
    check::{state::CheckState, SemanticToken, TokenKind},
    ir::{common::generic_args::GenericArgsIR, IrNode},
    item::common::type_::ContainsOffset,
    parser::top::struct_::Struct,
    util::Spanned,
};

use super::struct_body::StructBodyIR;

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct StructIR<'db> {
    pub name: Spanned<String>,
    pub generics: Spanned<GenericArgsIR<'db>>,
    pub body: Spanned<StructBodyIR<'db>>,
}

impl<'db> Struct {
    pub fn check(&self, state: &mut CheckState<'db>) -> StructIR<'db> {
        let generics = (self.generics.0.check(state), self.generics.1);
        let body = (self.body.0.check(state), self.body.1);
        StructIR {
            name: self.name.clone(),
            generics,
            body,
        }
    }
}

impl<'db> IrNode<'db> for StructIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut crate::ir::IrState<'db>) -> &dyn IrNode {
        if self.generics.1.contains_offset(offset) {
            return self.generics.0.at_offset(offset, state);
        }
        if self.body.1.contains_offset(offset) {
            return self.body.0.at_offset(offset, state);
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
            kind: TokenKind::Struct,
        });
        self.generics.0.tokens(tokens, state);
        self.body.0.tokens(tokens, state);
    }
}
