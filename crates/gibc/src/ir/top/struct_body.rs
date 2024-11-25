use crate::{
    check::state::CheckState,
    ir::{ty::TypeIR, ContainsOffset, IrNode},
    parser::top::struct_body::StructBody,
    util::Spanned,
};

use super::struct_field::StructFieldIR;

#[derive(Debug, PartialEq, Clone)]
pub enum StructBodyIR<'db> {
    None,
    Tuple(Vec<Spanned<TypeIR<'db>>>),
    Fields(Vec<Spanned<StructFieldIR<'db>>>),
}
impl<'db> StructBody {
    pub fn check(&self, state: &mut CheckState<'db>) -> StructBodyIR<'db> {
        match &self {
            StructBody::None => StructBodyIR::None,
            StructBody::Tuple(v) => {
                let tys = v.iter().map(|ty| (ty.0.check(state), ty.1)).collect();
                StructBodyIR::Tuple(tys)
            }
            StructBody::Fields(fields) => StructBodyIR::Fields(
                fields
                    .iter()
                    .map(|(field, span)| (field.check(state), *span))
                    .collect(),
            ),
        }
    }
}

impl<'db> IrNode<'db> for StructBodyIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut crate::ir::IrState<'db>) -> &dyn IrNode {
        match self {
            StructBodyIR::None => self,
            StructBodyIR::Tuple(tys) => {
                for (ty, span) in tys {
                    if span.contains_offset(offset) {
                        return ty.at_offset(offset, state);
                    }
                }
                self
            }
            StructBodyIR::Fields(fields) => {
                for (field, span) in fields {
                    if span.contains_offset(offset) {
                        return field.at_offset(offset, state);
                    }
                }
                self
            }
        }
    }

    fn tokens(
        &self,
        tokens: &mut Vec<crate::check::SemanticToken>,
        state: &mut crate::ir::IrState<'db>,
    ) {
        match self {
            StructBodyIR::None => {}
            StructBodyIR::Tuple(tys) => {
                for ty in tys {
                    ty.0.tokens(tokens, state);
                }
            }
            StructBodyIR::Fields(fields) => {
                for field in fields {
                    field.0.tokens(tokens, state);
                }
            }
        }
    }

    fn debug_name(&self) -> &'static str {
        "StructBodyIR"
    }
}
