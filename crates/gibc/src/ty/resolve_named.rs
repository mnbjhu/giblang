use crate::check::{is_scoped::IsScoped, scoped_state::Scoped};

use super::{FuncTy, Generic, Named, Ty};

impl<'db> Ty<'db> {
    pub fn resolve_named(&self, state: &impl IsScoped<'db>) -> Ty<'db> {
        match self {
            Ty::TypeVar { .. } | Ty::Any | Ty::Unknown | Ty::Nothing => self.clone(),
            Ty::Function(func) => Ty::Function(func.resolve_named(state)),
            Ty::Tuple(tys) => Ty::Tuple(tys.iter().map(|it| it.resolve_named(state)).collect()),
            Ty::Sum(tys) => Ty::Sum(tys.iter().map(|it| it.resolve_named(state)).collect()),
            Ty::Named(Named { name, .. }) => {
                if state.try_get_decl_path(*name).is_none() {
                    Ty::Unknown
                } else {
                    self.clone()
                }
            }
            Ty::Generic(g) => Ty::Generic(g.resolve_named(state)),
            Ty::Meta(inner) => Ty::Meta(Box::new(inner.resolve_named(state))),
        }
    }
}

impl<'db> FuncTy<'db> {
    pub fn resolve_named(&self, state: &impl IsScoped<'db>) -> FuncTy<'db> {
        FuncTy {
            receiver: self
                .receiver
                .as_ref()
                .map(|ty| Box::new(ty.resolve_named(state))),
            args: self.args.iter().map(|ty| ty.resolve_named(state)).collect(),
            ret: Box::new(self.ret.resolve_named(state)),
        }
    }
}

impl<'db> Generic<'db> {
    pub fn resolve_named(&self, state: &impl IsScoped<'db>) -> Generic<'db> {
        Generic {
            name: self.name.clone(),
            variance: self.variance,
            super_: Box::new(self.super_.resolve_named(state)),
        }
    }
}
