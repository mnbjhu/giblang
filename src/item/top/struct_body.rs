use crate::{
    check::{state::CheckState, SemanticToken},
    item::{common::type_::ContainsOffset, AstItem},
    parser::top::struct_body::StructBody,
};

impl AstItem for StructBody {
    fn at_offset<'me>(
        &'me self,
        state: &mut crate::check::state::CheckState,
        offset: usize,
    ) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        match self {
            StructBody::None => self,
            StructBody::Tuple(fields) => {
                for (field, span) in fields {
                    if span.contains_offset(offset) {
                        return field.at_offset(state, offset);
                    }
                }
                self
            }
            StructBody::Fields(fields) => {
                for (field, span) in fields {
                    if span.contains_offset(offset) {
                        return field.at_offset(state, offset);
                    }
                }
                self
            }
        }
    }

    fn tokens(&self, state: &mut CheckState, tokens: &mut Vec<SemanticToken>) {
        match self {
            StructBody::None => {}
            StructBody::Tuple(fields) => {
                for (field, _) in fields {
                    field.tokens(state, tokens);
                }
            }
            StructBody::Fields(fields) => {
                for (field, _) in fields {
                    field.tokens(state, tokens);
                }
            }
        }
    }
}
