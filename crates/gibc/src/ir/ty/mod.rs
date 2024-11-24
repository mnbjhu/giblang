use named::NamedTypeIR;

use crate::{
    check::{state::CheckState, SemanticToken, TokenKind},
    parser::common::type_::Type,
    ty::{FuncTy, Ty},
    util::{Span, Spanned},
};

use super::{top::TopIRData, ContainsOffset, IrNode, IrState};

pub mod named;

#[derive(Debug, PartialEq, Clone)]
pub struct TypeIR<'db> {
    pub data: TypeIRData<'db>,
    pub ty: Ty<'db>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypeIRData<'db> {
    Wildcard(Span),
    Named(NamedTypeIR<'db>),
    Tuple(Vec<Spanned<TypeIR<'db>>>),
    Sum(Vec<Spanned<TypeIR<'db>>>),
    Function {
        receiver: Option<Box<Spanned<TypeIR<'db>>>>,
        args: Vec<Spanned<TypeIR<'db>>>,
        ret: Box<Spanned<TypeIR<'db>>>,
    },
    Generic(Spanned<String>),
    Any(Span),
    Nothing(Span),
}

impl<'db> Type {
    pub fn check(&self, state: &mut CheckState<'db>) -> TypeIR<'db> {
        match &self {
            Type::Named(named) => named.check(state),
            Type::Tuple(tup) => {
                let mut tys = vec![];
                for (ty, span) in tup {
                    tys.push((ty.check(state), *span));
                }
                let ty = Ty::Tuple(tys.iter().map(|ir| ir.0.ty.clone()).collect());
                TypeIR {
                    data: TypeIRData::Tuple(tys),
                    ty,
                }
            }
            Type::Sum(tup) => {
                let mut tys = vec![];
                for (ty, span) in tup {
                    tys.push((ty.check(state), *span));
                }
                let ty = Ty::Sum(tys.iter().map(|ir| ir.0.ty.clone()).collect());
                TypeIR {
                    data: TypeIRData::Sum(tys),
                    ty,
                }
            }
            Type::Function {
                receiver,
                args,
                ret,
            } => {
                let receiver = receiver
                    .as_ref()
                    .map(|receiver| Box::new((receiver.0.check(state), receiver.1)));
                let mut arg_ir = vec![];
                for (arg, span) in args {
                    arg_ir.push((arg.check(state), *span));
                }
                let ret = (ret.0.check(state), ret.1);
                let ty = Ty::Function(FuncTy {
                    receiver: receiver.as_ref().map(|r| Box::new(r.0.ty.clone())),
                    args: arg_ir.iter().map(|ir| ir.0.ty.clone()).collect(),
                    ret: Box::new(ret.0.ty.clone()),
                });
                TypeIR {
                    data: TypeIRData::Function {
                        receiver,
                        args: arg_ir,
                        ret: Box::new(ret),
                    },
                    ty,
                }
            }
            Type::Wildcard(s) => {
                let id = state.type_state.new_type_var(*s, state.file_data);
                let ty = Ty::TypeVar { id };
                TypeIR {
                    data: TypeIRData::Wildcard(*s),
                    ty,
                }
            }
        }
    }
}

impl<'db> IrNode<'db> for TypeIR<'db> {
    #[allow(clippy::only_used_in_recursion)]
    fn at_offset(&self, offset: usize, state: &mut IrState<'db>) -> &dyn IrNode {
        match &self.data {
            TypeIRData::Tuple(tys) | TypeIRData::Sum(tys) => {
                for (ty, span) in tys {
                    if span.contains_offset(offset) {
                        return ty.at_offset(offset, state);
                    }
                }
                self
            }
            TypeIRData::Function {
                receiver: Some(ref receiver),
                ..
            } => {
                if receiver.1.contains_offset(offset) {
                    return receiver.as_ref().0.at_offset(offset, state);
                }
                self
            }
            TypeIRData::Function {
                args: ref arg_tys, ..
            } => {
                for (arg, span) in arg_tys {
                    if span.contains_offset(offset) {
                        return arg.at_offset(offset, state);
                    }
                }
                self
            }
            TypeIRData::Wildcard(_)
            | TypeIRData::Generic(_)
            | TypeIRData::Any(_)
            | TypeIRData::Nothing(_) => self,
            TypeIRData::Named(named) => named.at_offset(offset, state),
        }
    }

    fn tokens(&self, tokens: &mut Vec<SemanticToken>, state: &mut IrState<'db>) {
        match &self.data {
            TypeIRData::Named(named) => named.tokens(tokens, state),
            TypeIRData::Tuple(tys) | TypeIRData::Sum(tys) => {
                for (ty, _) in tys {
                    ty.tokens(tokens, state);
                }
            }
            TypeIRData::Function {
                receiver: Some(receiver),
                args,
                ret,
            } => {
                receiver.0.tokens(tokens, state);
                for (arg, _) in args {
                    arg.tokens(tokens, state);
                }
                ret.0.tokens(tokens, state);
            }
            TypeIRData::Function { args, ret, .. } => {
                if let Some(receiver) = args.first() {
                    receiver.0.tokens(tokens, state);
                }
                for (arg, _) in args {
                    arg.tokens(tokens, state);
                }
                ret.0.tokens(tokens, state);
            }
            TypeIRData::Wildcard(_) => {}
            TypeIRData::Generic((_, span)) => {
                tokens.push(SemanticToken {
                    span: *span,
                    kind: TokenKind::Generic,
                });
            }
            TypeIRData::Any(s) | TypeIRData::Nothing(s) => {
                tokens.push(SemanticToken {
                    span: *s,
                    kind: TokenKind::Struct,
                });
            }
        }
    }

    fn debug_name(&self) -> &'static str {
        "TypeIR"
    }
}
