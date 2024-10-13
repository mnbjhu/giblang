use crate::{item::AstItem, lexer::literal::Literal};

impl AstItem for Literal {
    fn at_offset<'me>(
        &'me self,
        _state: &mut crate::check::state::CheckState,
        _offset: usize,
    ) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        self
    }

    fn hover<'db>(
        &self,
        state: &mut crate::check::state::CheckState<'_, 'db>,
        offset: usize,
        type_vars: &std::collections::HashMap<u32, crate::ty::Ty<'db>>,
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
