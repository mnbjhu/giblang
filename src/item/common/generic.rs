use std::collections::HashMap;

use crate::{
    check::{state::CheckState, TokenKind},
    item::AstItem,
    parser::common::generic_arg::GenericArg,
    ty::Ty,
};

impl AstItem for GenericArg {
    fn at_offset<'me>(&'me self, state: &mut CheckState, offset: usize) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        if let Some(super_) = &self.super_ {
            if super_.1.start <= offset && offset <= super_.1.end {
                return super_.0.at_offset(state, offset);
            }
        }
        self
    }

    fn hover<'db>(
        &self,
        state: &mut CheckState,
        _: usize,
        type_vars: &HashMap<u32, Ty<'db>>,
    ) -> Option<String> {
        if let Some(super_) = &self.super_ {
            let ty = super_.0.check(state);
            Some(format!("{}: {}", self.name.0, ty.get_name(state)))
        } else {
            Some(self.name.0.clone())
        }
    }

    fn tokens(&self, state: &mut CheckState, tokens: &mut Vec<crate::check::SemanticToken>) {
        self.check(state);
        tokens.push(crate::check::SemanticToken {
            span: self.name.1,
            kind: TokenKind::Generic,
        });
    }
}
