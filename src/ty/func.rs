use std::collections::HashMap;

use crate::{
    check::state::CheckState,
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
    ) -> Option<(IdentDef<'db>, FuncTy<'db>)> {
        if let Ty::Generic(Generic { super_, .. }) = state.resolved_ty(self) {
            state.resolved_ty(&super_).get_func(name, state, &super_)
        } else {
            state.resolved_ty(self).get_func(name, state, self)
        }
        // let mut funcs = get_sub_tys(self, state, name.1)
        //     .iter()
        //     .filter_map(|ty| ty.get_func(name, state, self))
        //     .collect::<Vec<_>>();
        // funcs.extend(self.get_func(name, state, self));
        // if funcs.len() > 1 {
        //     state.simple_error(&format!("Ambiguous call to function {}", &name.0), name.1);
        //     None
        // } else if funcs.len() == 1 {
        //     let func = (
        //         funcs[0].0.clone(),
        //         funcs[0].1.inst(&mut HashMap::new(), state, name.1),
        //     );
        //     Some(func)
        // } else {
        //     let ident = check_ident(&[name.clone()], state);
        //     let ExprIRData::Ident(segs) = ident.data else {
        //         panic!("Expected ident...")
        //     };
        //     let def = segs.last().unwrap().0.clone();
        //     if let Ty::Function(func_ty) = ident.ty {
        //         Some((def, func_ty))
        //     } else {
        //         None
        //     }
        // }
    }

    pub fn get_matching_member_func(
        &self,
        name: &Spanned<String>,
        state: &mut CheckState<'db>,
        args: &Vec<Spanned<Ty<'db>>>,
    ) -> Vec<(IdentDef<'db>, FuncTy<'db>)> {
        let mut funcs = get_sub_tys(self, state, name.1)
            .iter()
            .filter_map(|ty| ty.get_func(name, state, self))
            .collect::<Vec<_>>();
        funcs.extend(self.get_func(name, state, self));
        for (index, arg) in args.iter().enumerate() {
            if funcs.len() <= 1 {
                break;
            }
            if funcs.iter().any(|func| {
                if let Some(expected) = func.1.args.get(index) {
                    arg.0.check_is_instance_of(expected, state, arg.1)
                } else {
                    false
                }
            }) {
                funcs.retain(|func| {
                    if let Some(expected) = func.1.args.get(index) {
                        arg.0.check_is_instance_of(expected, state, arg.1)
                    } else {
                        false
                    }
                });
            }
        }
        funcs
    }

    pub fn member_funcs(
        &self,
        state: &mut CheckState<'db>,
        span: Span,
    ) -> Vec<(Decl<'db>, FuncTy<'db>)> {
        let mut funcs = get_sub_tys(self, state, span)
            .iter()
            .flat_map(|t| t.get_funcs(state))
            .collect::<Vec<_>>();
        funcs.extend(self.get_funcs(state));
        funcs
    }

    pub fn fields(&self, state: &mut CheckState<'db>) -> Vec<(String, Ty<'db>)> {
        let Ty::Named(Named { name, args }) = &self.clone().try_resolve(state) else {
            return Vec::new();
        };
        let Some(decl) = state.try_get_decl_path(*name) else {
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
