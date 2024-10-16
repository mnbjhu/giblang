use crate::{
    check::{state::CheckState, SemanticToken},
    item::AstItem,
    parser::common::type_::Type,
    util::Span,
};

impl AstItem for Type {
    fn at_offset<'me>(&'me self, state: &mut CheckState, offset: usize) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        match self {
            Type::Named(named) => return named.at_offset(state, offset),
            Type::Sum(tys) | Type::Tuple(tys) => {
                for (ty, span) in tys {
                    if span.contains_offset(offset) {
                        return ty.at_offset(state, offset);
                    }
                }
            }
            Type::Function {
                receiver,
                args,
                ret,
            } => {
                if let Some(receiver) = receiver {
                    if receiver.1.contains_offset(offset) {
                        return receiver.0.at_offset(state, offset);
                    }
                }
                for (ty, span) in args {
                    if span.contains_offset(offset) {
                        return ty.at_offset(state, offset);
                    }
                }
                let (ret_ty, ret_span) = ret.as_ref();
                if ret_span.contains_offset(offset) {
                    return ret_ty.at_offset(state, offset);
                }
            }
            Type::Wildcard(_) => {}
        };
        self
    }

    fn tokens(&self, state: &mut CheckState, tokens: &mut Vec<SemanticToken>) {
        match self {
            Type::Named(named) => named.tokens(state, tokens),
            Type::Sum(tys) | Type::Tuple(tys) => {
                for (ty, _) in tys {
                    ty.tokens(state, tokens);
                }
            }
            Type::Function {
                receiver,
                args,
                ret,
            } => {
                if let Some(receiver) = receiver {
                    receiver.0.tokens(state, tokens);
                }
                for (ty, _) in args {
                    ty.tokens(state, tokens);
                }
                ret.0.tokens(state, tokens);
            }
            Type::Wildcard(_) => {}
        }
    }
}

pub trait ContainsOffset {
    fn contains_offset(&self, offset: usize) -> bool;
}

impl ContainsOffset for Span {
    fn contains_offset(&self, offset: usize) -> bool {
        self.start <= offset && offset <= self.end
    }
}
