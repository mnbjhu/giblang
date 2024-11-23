use std::collections::HashMap;

use crate::{
    check::{
        build_state::BuildState,
        err::{is_not_instance::IsNotInstance, CheckError},
        state::CheckState,
    },
    db::{
        decl::{Decl, DeclKind},
        path::ModulePath,
    },
    item::definitions::ident::IdentDef,
    parser::common::variance::Variance,
    ty::Ty,
    util::{Span, Spanned},
};

use super::{sub_tys::path_to_sub_ty, FuncTy, Generic, Named};

impl<'db> Ty<'db> {
    pub fn expect_is_instance_of(
        &self,
        other: &Ty<'db>,
        state: &mut CheckState<'db>,
        span: Span,
    ) -> bool {
        if self.eq(other) {
            return true;
        }
        let res = match (self, other) {
            (Ty::Unknown | Ty::Nothing, _) | (_, Ty::Unknown) => true,
            (Ty::TypeVar { id }, other) | (other, Ty::TypeVar { id }) => {
                state.expected_var_is_ty(*id, other.clone(), span);
                true
            }
            (_, Ty::Any) => true,
            (Ty::Named(Named { .. }), Ty::Named(Named { .. })) => {
                expect_named_is_instance_of_named(self, other, state, span)
            }
            (_, Ty::Sum(tys)) => tys
                .iter()
                .all(|other| self.expect_is_instance_of(other, state, span)),
            // TODO: Fix or remove
            (Ty::Sum(tys), _) => tys
                .iter()
                .any(|ty| ty.expect_is_instance_of(other, state, span)),
            (Ty::Tuple(v), Ty::Tuple(other)) => {
                v.len() == other.len()
                    && v.iter()
                        .zip(other)
                        .all(|(s, o)| s.expect_is_instance_of(o, state, span))
            }
            (Ty::Generic(Generic { super_, .. }), _) => {
                // TODO: Fix error messages for generic types
                return super_.expect_is_instance_of(other, state, span);
            }
            (_, Ty::Generic(Generic { super_, name, .. })) => {
                if name.0 == "Self" {
                    return self.expect_is_instance_of(super_, state, span);
                }
                self.eq(other)
            }
            (Ty::Function(ty), Ty::Function(other)) => ty.expect_is_instance_of(other, state, span),
            _ => false,
        };
        if !res {
            state.error(CheckError::IsNotInstance(IsNotInstance {
                span,
                found: self.get_name(state, None),
                expected: other.get_name(state, None),
                file: state.file_data,
            }));
        }
        res
    }

    pub fn imply_named_sub_ty(
        &self,
        sub_ty: ModulePath<'db>,
        state: &mut CheckState<'db>,
    ) -> Option<Ty<'db>> {
        if let Ty::Named(Named { name, .. }) = self {
            let path = path_to_sub_ty(*name, sub_ty, state)?;
            let mut ty = self.clone();
            for impl_ in path {
                ty = impl_.map(state.db, &ty);
            }
            Some(ty)
        } else {
            None
        }
    }
}

impl<'db> FuncTy<'db> {
    pub fn expect_is_instance_of(
        &self,
        other: &FuncTy<'db>,
        state: &mut CheckState<'db>,
        span: Span,
    ) -> bool {
        self.receiver.as_ref().map_or(true, |r| {
            other
                .receiver
                .as_ref()
                .map_or(false, |o| r.expect_is_instance_of(o, state, span))
        }) && self
            .args
            .iter()
            .zip(&other.args)
            .all(|(first, second)| first.expect_is_instance_of(second, state, span))
            && self.ret.expect_is_instance_of(&other.ret, state, span)
    }
}

impl<'db> Ty<'db> {
    pub fn get_all_impl_funcs(
        &self,
        name: &Spanned<String>,
        state: &mut CheckState<'db>,
    ) -> Vec<(IdentDef<'db>, FuncTy<'db>)> {
        if let Ty::Named(Named { name: id, .. }) = self {
            state
                .project
                .get_impls(state.db, *id)
                .into_iter()
                .flat_map(|i| {
                    let mut funcs = Vec::new();
                    let mut implied = HashMap::new();
                    i.from_ty(state.db).imply_generic_args(self, &mut implied);
                    let self_ty = self.parameterize(&implied);
                    implied.insert("Self".to_string(), self_ty);
                    if i.to_ty(state.db).is_some() {
                        let new = i.map(state.db, self);
                        funcs.extend(new.get_all_impl_funcs(name, state));
                    }
                    for func in i.functions(state.db) {
                        if func.name(state.db) != name.0 {
                            continue;
                        }
                        let ty = func.get_ty(state).parameterize(&implied);
                        if let Ty::Function(ty) = ty {
                            funcs.push((IdentDef::Decl(*func), ty));
                        }
                    }
                    funcs
                })
                .collect::<Vec<_>>()
        } else {
            vec![]
        }
    }

    pub fn get_trait_func_decls(&self, state: &mut BuildState<'db>) -> Vec<(Decl<'db>, Decl<'db>)> {
        let Ty::Named(Named { name, .. }) = self else {
            return Vec::new();
        };
        let impls = state
            .project
            .get_impls(state.db, *name)
            .into_iter()
            .filter(|i| i.to_ty(state.db).is_some())
            .flat_map(|i| {
                let Ty::Named(Named { name, .. }) = i.to_ty(state.db).unwrap() else {
                    unreachable!()
                };
                let DeclKind::Trait { body, .. } = state
                    .project
                    .get_decl(state.db, name)
                    .unwrap()
                    .kind(state.db)
                else {
                    unreachable!()
                };
                let mut found = vec![];
                for trait_func in body {
                    let impl_func = i
                        .functions(state.db)
                        .iter()
                        .find(|f| f.name(state.db) == trait_func.name(state.db));
                    if let Some(impl_func) = impl_func {
                        found.push((*trait_func, *impl_func));
                    }
                }
                found.extend(i.map(state.db, self).get_trait_func_decls(state));
                found
            })
            .collect();
        impls
    }

    pub fn get_trait_funcs(
        &self,
        name: &Spanned<String>,
        state: &mut CheckState<'db>,
        self_ty: &Ty<'db>,
    ) -> Vec<(IdentDef<'db>, FuncTy<'db>)> {
        if let Ty::Named(Named { name: id, args }) = self {
            if let Some(DeclKind::Trait { body, generics }) =
                state.try_get_decl_path(*id).map(|d| d.kind(state.db))
            {
                if args.len() != generics.len() {
                    return vec![];
                }
                let mut params = generics
                    .iter()
                    .map(|arg| arg.name.0.clone())
                    .zip(args.iter().cloned())
                    .collect::<HashMap<_, _>>();
                params.insert("Self".to_string(), self_ty.clone());
                return body
                    .iter()
                    .filter(|func| func.name(state.db) == name.0)
                    .map(|func| {
                        let Ty::Function(ty) =
                            func.get_ty(state).parameterize(&params).inst(state, name.1)
                        else {
                            panic!("Expected function");
                        };
                        (IdentDef::Decl(*func), ty)
                    })
                    .collect::<Vec<_>>();
            };
        }
        Vec::new()
    }

    pub fn get_func(
        &self,
        name: &Spanned<String>,
        state: &mut CheckState<'db>,
        self_ty: &Ty<'db>,
    ) -> Option<(IdentDef<'db>, FuncTy<'db>)> {
        if matches!(self, Ty::Named(_)) {
            let mut funcs = self.get_trait_funcs(name, state, self_ty);
            funcs.extend(self.get_all_impl_funcs(name, state));
            funcs.pop()
        } else {
            None
        }
    }

    pub fn get_funcs(&self, state: &CheckState<'db>) -> Vec<(Decl<'db>, FuncTy<'db>)> {
        let mut funcs = Vec::new();
        if let Ty::Named(Named { name, .. }) = self {
            if let Some(DeclKind::Trait { body, .. }) =
                state.try_get_decl_path(*name).map(|d| d.kind(state.db))
            {
                funcs.extend(body.iter().map(|func_decl| {
                    let Ty::Function(func) = func_decl.get_ty(state) else {
                        panic!("Expected function");
                    };
                    (*func_decl, func)
                }));
            }
            let impls = state
                .project
                .get_impls(state.db, *name)
                .into_iter()
                .filter(|i| {
                    let Ty::Named(Named { name: n, .. }) = i.from_ty(state.db) else {
                        unreachable!()
                    };
                    *name == n && i.to_ty(state.db).is_none()
                })
                .flat_map(|i| {
                    let mut implied = HashMap::new();
                    i.from_ty(state.db).imply_generic_args(self, &mut implied);
                    let self_ty = self.parameterize(&implied);
                    implied.insert("Self".to_string(), self_ty);
                    i.functions(state.db)
                        .iter()
                        .map(|f| {
                            let DeclKind::Function(func) = f.kind(state.db) else {
                                panic!("Expected function");
                            };
                            (*f, func.get_ty().parameterize(&implied))
                        })
                        .collect::<Vec<_>>()
                });
            funcs.extend(impls);
        }
        funcs
    }
}

fn expect_named_is_instance_of_named<'db>(
    first: &Ty<'db>,
    second: &Ty<'db>,
    state: &mut CheckState<'db>,
    span: Span,
) -> bool {
    if let (
        Ty::Named(Named { name, .. }),
        Ty::Named(Named {
            name: other_name,
            args: other_args,
        }),
    ) = (first, second)
    {
        if let Some(Ty::Named(Named {
            args: implied_args, ..
        })) = first.imply_named_sub_ty(*other_name, state)
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
    } else {
        panic!("Expected named types")
    }
}
