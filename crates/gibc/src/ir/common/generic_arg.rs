use crate::{
    check::{state::CheckState, SemanticToken, TokenKind},
    ir::{ty::TypeIR, ContainsOffset, IrNode, IrState},
    parser::common::{generic_arg::GenericArg, variance::Variance},
    ty::{Generic, Ty},
    util::Spanned,
};

#[derive(Debug, PartialEq, Clone)]
pub struct GenericArgIR<'db> {
    pub name: Spanned<String>,
    pub variance: Variance,
    pub super_: Option<Spanned<TypeIR<'db>>>,
}
impl<'db> GenericArgIR<'db> {
    pub fn get_ty(&self) -> Ty<'db> {
        Ty::Generic(Generic {
            name: self.name.clone(),
            variance: self.variance,
            super_: Box::new(self.super_.as_ref().map_or(Ty::Any, |sup| sup.0.ty.clone())),
        })
    }
}

impl<'db> GenericArg {
    pub fn check(&self, state: &mut CheckState<'db>) -> GenericArgIR<'db> {
        let super_ = self
            .super_
            .as_ref()
            .map(|(super_, span)| (super_.check(state), *span));

        let generic = Generic {
            name: self.name.clone(),
            variance: self.variance,
            super_: Box::new(
                super_
                    .as_ref()
                    .map_or(Ty::Any, |super_| super_.0.ty.clone()),
            ),
        };
        state.insert_generic(self.name.0.to_string(), generic.clone());
        GenericArgIR {
            name: self.name.clone(),
            variance: self.variance,
            super_,
        }
    }
}

impl<'db> IrNode<'db> for GenericArgIR<'db> {
    fn tokens(&self, tokens: &mut Vec<SemanticToken>, state: &mut IrState<'db>) {
        tokens.push(SemanticToken {
            kind: TokenKind::Generic,
            span: self.name.1,
        });
        if let Some(super_) = &self.super_ {
            super_.0.tokens(tokens, state);
        }
    }

    fn at_offset(&self, offset: usize, state: &mut IrState<'db>) -> &dyn IrNode {
        if let Some(super_) = &self.super_ {
            if super_.1.contains_offset(offset) {
                return super_.0.at_offset(offset, state);
            }
        }
        self
    }
}
