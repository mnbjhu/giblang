use crate::{
    check::state::CheckState,
    db::path::ModulePath,
    parser::common::variance::Variance,
    util::Span,
};

use super::{is_instance::path_to_sub_ty, FuncTy, Generic, Named, Ty};

impl<'db> Ty<'db> {
    pub fn expect_is(&self, other: &Ty<'db>, state: &mut CheckState<'db>, span: Span) -> bool {
        if self.eq(other) {
            return true;
        }
        match (self, other) {
            (Ty::Unknown | Ty::Nothing, _) | (_, Ty::Unknown) => true,
            (Ty::TypeVar { id }, _) => {
                let resolved = state.get_resolved_type_var(*id);
                if let Ty::Unknown = resolved {
                    false
                } else {
                    resolved.expect_is(other, state, span)
                }
            }
            (_, Ty::TypeVar { id }) => {
                let resolved = state.get_resolved_type_var(*id);
                if let Ty::Unknown = resolved {
                    false
                } else {
                    self.expect_is(&resolved, state, span)
                }
            }
            (_, Ty::Any) => true,
            (Ty::Named(s), Ty::Named(other)) => s.expect_is(other, state, span),
            (_, Ty::Sum(tys)) => tys.iter().all(|other| self.expect_is(other, state, span)),
            // TODO: Fix or remove
            (Ty::Sum(tys), _) => tys.iter().any(|ty| ty.expect_is(other, state, span)),
            (Ty::Tuple(v), Ty::Tuple(other)) => {
                v.len() == other.len()
                    && v.iter()
                        .zip(other)
                        .all(|(s, o)| s.expect_is(o, state, span))
            }
            (Ty::Generic(Generic { super_, .. }), _) => {
                // TODO: Fix error messages for generic types
                super_.expect_is(other, state, span)
            }
            (_, Ty::Generic(Generic { super_, name, .. })) => {
                if name.0 == "Self" {
                    return self.expect_is(super_, state, span);
                }
                self.eq(other)
            }
            (Ty::Function(ty), Ty::Function(other)) => ty.expect_is(other, state, span),
            _ => false,
        }
    }
}

impl<'db> FuncTy<'db> {
    pub fn expect_is(&self, other: &FuncTy<'db>, state: &mut CheckState<'db>, span: Span) -> bool {
        self.receiver.as_ref().map_or(true, |r| {
            other
                .receiver
                .as_ref()
                .map_or(false, |o| r.expect_is(o, state, span))
        }) && self
            .args
            .iter()
            .zip(&other.args)
            .all(|(first, second)| first.expect_is(second, state, span))
            && self.ret.expect_is(&other.ret, state, span)
    }
}

impl<'db> Named<'db> {
    fn expect_is(&self, second: &Named<'db>, state: &mut CheckState<'db>, span: Span) -> bool {
        let (
            Named { name, .. },
            Named {
                name: other_name,
                args: other_args,
            },
        ) = (self, second);

        if let Some(Named {
            args: implied_args, ..
        }) = self.imply_named_sub_ty(*other_name, state)
        {
            // TODO: Check this unwrap
            let decl = state.try_get_decl_path(*name).unwrap();
            let generics = decl.generics(state.db);
            for ((g, arg), other) in generics.iter().zip(implied_args).zip(other_args) {
                let variance = g.variance;
                match variance {
                    Variance::Invariant => {
                        arg.expect_is_instance_of(other, state, span);
                        other.expect_is_instance_of(&arg, state, span);
                    }
                    Variance::Covariant => {
                        arg.expect_is_instance_of(other, state, span);
                    }
                    Variance::Contravariant => {
                        other.expect_is_instance_of(&arg, state, span);
                    }
                }
            }
            true
        } else {
            false
        }
    }
    fn imply_named_sub_ty(
        &self,
        sub_ty: ModulePath<'db>,
        state: &mut CheckState<'db>,
    ) -> Option<Named<'db>> {
        let Named { name, .. } = self;
        let path = path_to_sub_ty(*name, sub_ty, state)?;
        let mut ty = Ty::Named(self.clone());
        for impl_ in path {
            ty = impl_.map(state.db, &ty);
        }
        if let Ty::Named(named) = ty {
            Some(named)
        } else {
            unreachable!()
        }
    }
}

impl<'db> Named<'db> {}
