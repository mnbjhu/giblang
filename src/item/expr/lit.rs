use std::collections::HashMap;

use crate::{check::state::CheckState, item::AstItem, lexer::literal::Literal};

impl AstItem for Literal {
    fn at_offset<'me>(&'me self, _state: &mut CheckState, _offset: usize) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        self
    }

    fn hover<'db>(
        &self,
        _: &mut CheckState<'_, 'db>,
        _: usize,
        _: &HashMap<u32, crate::ty::Ty<'db>>,
    ) -> Option<String> {
        let name = match self {
            Literal::Int(_) => "Int",
            Literal::Float(_) => "Float",
            Literal::String(_) => "String",
            Literal::Char(_) => "Char",
            Literal::Bool(_) => "Bool",
        };
        Some(name.to_string())
    }
}
