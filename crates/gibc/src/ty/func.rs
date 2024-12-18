use std::collections::HashMap;

use crate::{
    check::{is_scoped::IsScoped, scoped_state::Scoped, state::CheckState},
    db::decl::{struct_::StructDecl, Decl, DeclKind},
    item::definitions::ident::IdentDef,
    util::{Span, Spanned},
};

use super::{sub_tys::get_sub_tys, FuncTy, Generic, Named, Ty};

impl<'db> Ty<'db> {
    pub fn try_get_func_ty(&self, state: &mut CheckState<'db>, span: Span) -> Option<FuncTy<'db>> {
        if let Ty::Function(func_ty) = self {
            Some(func_ty.clone())
        } else if let Ty::Meta(ty) = self {
            if let Ty::Named(Named { name, .. }) = ty.as_ref() {
                let decl = state.try_get_decl_path(*name);
                if let Some(decl) = decl {
                    if let DeclKind::Struct { body, .. } = decl.kind(state.db()) {
                        return body
                            .get_constructor_ty(ty.as_ref().clone())
                            .map(|ty| ty.inst(&mut HashMap::new(), state, span));
                    }
                }
            }
            None
        } else {
            None
        }
    }
    pub fn get_member_func(
        &self,
        name: &Spanned<String>,
        state: &mut CheckState<'db>,
    ) -> Option<(IdentDef<'db>, FuncTy<'db>)> {
        if let Ty::Generic(Generic { super_, .. }) = state.resolved_ty(self) {
            state.resolved_ty(&super_).get_func(name, state, &super_)
        } else {
            state.resolved_ty(self).get_func(name, state, self)
        }
    }

    pub fn member_funcs(&self, state: &impl IsScoped<'db>) -> Vec<(Decl<'db>, FuncTy<'db>)> {
        let mut funcs = get_sub_tys(self, state)
            .iter()
            .flat_map(|t| t.get_funcs(state))
            .collect::<Vec<_>>();
        funcs.extend(self.get_funcs(state));
        funcs
    }

    pub fn fields(&self, state: &impl IsScoped<'db>) -> Vec<(String, Ty<'db>)> {
        let Ty::Named(Named { name, args }) = &self.clone().try_resolve(state) else {
            return Vec::new();
        };
        let Some(decl) = state.try_get_decl_path(*name) else {
            return Vec::new();
        };
        let DeclKind::Struct { body, generics } = decl.kind(state.db()) else {
            return Vec::new();
        };
        if generics.len() != args.len() {
            return Vec::new();
        }
        let params = generics
            .iter()
            .map(|arg| arg.name.0.clone())
            .zip(args.iter().cloned())
            .collect::<HashMap<_, _>>();
        match body {
            StructDecl::Fields(fields) => fields
                .iter()
                .map(|(name, ty)| (name.clone(), ty.parameterize(&params)))
                .collect(),
            StructDecl::Tuple(tys) => tys
                .iter()
                .enumerate()
                .map(|(i, ty)| (i.to_string(), ty.clone()))
                .collect(),
            StructDecl::None => vec![],
        }
    }
}
