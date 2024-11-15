use crate::{
    check::{state::CheckState, SemanticToken, TokenKind},
    ir::{ty::TypeIR, IrNode},
    item::common::type_::ContainsOffset as _,
    parser::top::struct_field::StructField,
    util::Spanned,
};

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct StructFieldIR<'db> {
    pub name: Spanned<String>,
    pub ty: Spanned<TypeIR<'db>>,
}

impl<'db> StructField {
    pub fn check(&self, state: &mut CheckState<'db>) -> StructFieldIR<'db> {
        let ty = (self.ty.0.check(state), self.ty.1);
        StructFieldIR {
            name: self.name.clone(),
            ty,
        }
    }
}

impl<'db> IrNode<'db> for StructFieldIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut crate::ir::IrState<'db>) -> &dyn IrNode {
        if self.ty.1.contains_offset(offset) {
            return self.ty.0.at_offset(offset, state);
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
            kind: TokenKind::Property,
        });
        self.ty.0.tokens(tokens, state);
    }
}
