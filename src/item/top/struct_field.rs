use std::collections::HashMap;

use crate::{
    check::{state::CheckState, SemanticToken, TokenKind},
    item::AstItem,
    parser::top::struct_field::StructField,
    ty::Ty,
};

impl AstItem for StructField {
    fn tokens(&self, _: &mut CheckState, tokens: &mut Vec<SemanticToken>, _: &Ty<'_>) {
        tokens.push(SemanticToken {
            span: self.name.1,
            kind: TokenKind::Property,
        });
    }

    fn hover<'db>(
        &self,
        state: &mut CheckState<'db>,
        _: usize,
        type_vars: &HashMap<u32, Ty<'db>>,
        ty: &Ty<'_>,
    ) -> Option<String> {
        Some(format!(
            "{}: {}",
            self.name.0,
            ty.get_name(state, Some(type_vars))
        ))
    }

    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        allocator
            .text(&self.name.0)
            .append(":")
            .append(allocator.space())
            .append(self.ty.0.pretty(allocator))
    }
}
