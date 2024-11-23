use crate::{
    check::{err::CheckError, scoped_state::Scoped as _, state::CheckState, SemanticToken},
    ir::{common::pattern::SpannedQualifiedNameIR, ContainsOffset, IrNode, IrState},
    parser::common::type_::NamedType,
    ty::{Named, Ty},
    util::{Span, Spanned},
};

use super::{TypeIR, TypeIRData};

#[derive(Debug, PartialEq, Clone)]
pub struct NamedTypeIR<'db> {
    pub name: SpannedQualifiedNameIR<'db>,
    pub args: Vec<Spanned<TypeIR<'db>>>,
}

impl<'db> NamedType {
    pub fn check(&self, state: &mut CheckState<'db>) -> TypeIR<'db> {
        if self.name.len() == 1 {
            let name = self.name[0].clone();
            if name.0 == "Any" {
                return TypeIR {
                    data: TypeIRData::Any(name.1),
                    ty: Ty::Any,
                };
            }
            if name.0 == "Nothing" {
                return TypeIR {
                    data: TypeIRData::Nothing(name.1),
                    ty: Ty::Nothing,
                };
            }
            if let Some(generic) = state.get_generic(&name.0).cloned() {
                let ty = Ty::Generic(generic);
                return TypeIR {
                    data: TypeIRData::Generic(name),
                    ty,
                };
            };
        };
        match state.get_decl_with_error(&self.name) {
            Ok(decl) => {
                let mut args = vec![];
                // TODO: Check generic bounds
                for (arg, _gen) in self.args.iter().zip(decl.generics(state.db)) {
                    args.push((arg.0.check(state), arg.1));
                }
                let arg_tys = args.iter().map(|ir| ir.0.ty.clone()).collect();
                let ty = Ty::Named(Named {
                    name: decl.path(state.db),
                    args: arg_tys,
                });
                TypeIR {
                    data: TypeIRData::Named(NamedTypeIR {
                        name: state.get_ident_ir(&self.name),
                        args,
                    }),
                    ty,
                }
            }
            Err(err) => {
                state.error(CheckError::Unresolved(err));
                let args = self
                    .args
                    .iter()
                    .map(|arg| (arg.0.check(state), arg.1))
                    .collect();
                TypeIR {
                    data: TypeIRData::Named(NamedTypeIR {
                        name: state.get_ident_ir(&self.name),
                        args,
                    }),
                    ty: Ty::Unknown,
                }
            }
        }
    }
}

impl<'db> IrNode<'db> for NamedTypeIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut crate::ir::IrState<'db>) -> &dyn IrNode {
        let ident_span: Span =
            (self.name.first().unwrap().1.start..self.name.last().unwrap().1.end).into();
        if ident_span.contains_offset(offset) {
            return self.name.at_offset(offset, state);
        }
        for (arg, span) in &self.args {
            if span.contains_offset(offset) {
                return arg.at_offset(offset, state);
            }
        }
        self
    }

    fn tokens(&self, tokens: &mut Vec<SemanticToken>, state: &mut IrState<'db>) {
        self.name.tokens(tokens, state);
        for (arg, _) in &self.args {
            arg.tokens(tokens, state);
        }
    }
}
