use std::collections::HashMap;

use crate::{check::{expr::ident::check_ident, state::CheckState}, db::decl::{struct_::StructDecl, DeclKind}, util::{Span, Spanned}};

use super::{is_instance::get_sub_tys, FuncTy, Ty};


impl<'db> Ty<'db> {
    pub fn try_get_func_ty(&self, state: &mut CheckState<'db>, span: Span) -> Option<FuncTy<'db>> {
        if let Ty::Function(func_ty) = self {
            Some(func_ty.clone())
        } else if let Ty::Meta(ty) = self {
            if let Ty::Named { name, .. } = ty.as_ref() {
                let decl = state.try_get_decl(*name);
                if let Some(decl) = decl {
                    if let DeclKind::Struct { body, .. } = decl.kind(state.db) {
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
    ) -> Option<FuncTy<'db>> {
        let mut funcs = get_sub_tys(self, state)
            .iter()
            .filter_map(|ty| ty.get_func(name, state))
            .collect::<Vec<_>>();
        funcs.extend(self.get_func(name, state));
        if funcs.len() > 1 {
            state.simple_error(&format!("Ambiguous call to function {}", &name.0), name.1);
            None
        } else if funcs.len() == 1 {
            let func = funcs[0].inst(&mut HashMap::new(), state, name.1);
            Some(func)
        } else if let Ty::Function(func_ty) = check_ident(state, &[name.clone()]) {
            Some(func_ty)
        } else {
            None
        }
    }

    pub fn member_funcs(&self, state: &mut CheckState<'db>) -> Vec<(String, FuncTy<'db>)> {
        let mut funcs = get_sub_tys(self, state)
            .iter()
            .flat_map(|t| t.get_funcs(state))
            .collect::<Vec<_>>();
        funcs.extend(self.get_funcs(state));
        funcs
    }

    pub fn fields(&self, state: &mut CheckState<'db>) -> Vec<(String, Ty<'db>)> {
        let Ty::Named { name, args } = self else {
            return Vec::new();
        };
        let Some(decl) = state.try_get_decl(*name) else {
            return Vec::new();
        };
        let DeclKind::Struct { body, generics } = decl.kind(state.db) else {
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