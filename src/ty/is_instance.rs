use std::collections::HashMap;

use crate::{
    check::{
        err::{is_not_instance::IsNotInstance, CheckError},
        state::CheckState,
    },
    db::{
        decl::{impl_::ImplForDecl, Decl, DeclKind},
        path::ModulePath,
    },
    item::definitions::ident::IdentDef,
    parser::common::variance::Variance,
    ty::Ty,
    util::{Span, Spanned},
};

use super::{FuncTy, Generic, Named};

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
            (Ty::TypeVar { id }, _) => {
                state.expected_var_is_ty(*id, other.clone(), span);
                true
            }
            (_, Ty::TypeVar { id }) => {
                state.expected_ty_is_var(*id, self.clone(), span);
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

    pub fn check_is_instance_of(
        &self,
        other: &Ty<'db>,
        state: &mut CheckState<'db>,
        span: Span,
    ) -> bool {
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
                    resolved.check_is_instance_of(other, state, span)
                }
            }
            (_, Ty::TypeVar { id }) => {
                let resolved = state.get_resolved_type_var(*id);
                if let Ty::Unknown = resolved {
                    false
                } else {
                    self.check_is_instance_of(&resolved, state, span)
                }
            }
            (_, Ty::Any) => true,
            (Ty::Named(Named { .. }), Ty::Named(Named { .. })) => {
                check_named_is_instance_of_named(self, other, state, span)
            }
            (_, Ty::Sum(tys)) => tys
                .iter()
                .all(|other| self.check_is_instance_of(other, state, span)),
            // TODO: Fix or remove
            (Ty::Sum(tys), _) => tys
                .iter()
                .any(|ty| ty.check_is_instance_of(other, state, span)),
            (Ty::Tuple(v), Ty::Tuple(other)) => {
                v.len() == other.len()
                    && v.iter()
                        .zip(other)
                        .all(|(s, o)| s.check_is_instance_of(o, state, span))
            }
            (Ty::Generic(Generic { super_, .. }), _) => {
                // TODO: Fix error messages for generic types
                super_.check_is_instance_of(other, state, span)
            }
            (_, Ty::Generic(Generic { super_, name, .. })) => {
                if name.0 == "Self" {
                    return self.check_is_instance_of(super_, state, span);
                }
                self.eq(other)
            }
            (Ty::Function(ty), Ty::Function(other)) => ty.check_is_instance_of(other, state, span),
            _ => false,
        }
    }

    fn imply_named_sub_ty(
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

    pub fn check_is_instance_of(
        &self,
        other: &FuncTy<'db>,
        state: &mut CheckState<'db>,
        span: Span,
    ) -> bool {
        self.receiver.as_ref().map_or(true, |r| {
            other
                .receiver
                .as_ref()
                .map_or(false, |o| r.check_is_instance_of(o, state, span))
        }) && self
            .args
            .iter()
            .zip(&other.args)
            .all(|(first, second)| first.check_is_instance_of(second, state, span))
            && self.ret.check_is_instance_of(&other.ret, state, span)
    }
}

impl<'db> Ty<'db> {
    pub fn get_func(
        &self,
        name: &Spanned<String>,
        state: &mut CheckState<'db>,
        self_ty: &Ty<'db>,
    ) -> Option<(IdentDef<'db>, FuncTy<'db>)> {
        if let Ty::Named(Named { name: id, args }) = self {
            if let Some(DeclKind::Trait { body, generics }) =
                state.try_get_decl_path(*id).map(|d| d.kind(state.db))
            {
                if args.len() != generics.len() {
                    return None;
                }
                let mut params = generics
                    .iter()
                    .map(|arg| arg.name.0.clone())
                    .zip(args.iter().cloned())
                    .collect::<HashMap<_, _>>();
                params.insert("Self".to_string(), self_ty.clone());
                let found = body
                    .iter()
                    .find(|func| func.name(state.db) == name.0)
                    .map(|func| {
                        let Ty::Function(ty) =
                            func.get_ty(state).parameterize(&params).inst(state, name.1)
                        else {
                            panic!("Expected function");
                        };
                        (IdentDef::Decl(*func), ty)
                    });
                if let Some(found) = found {
                    return Some(found);
                }
            };
            let mut impl_funcs = state
                .project
                .get_impls(state.db, *id)
                .into_iter()
                .filter(|i| {
                    let Ty::Named(Named { name: n, .. }) = i.from_ty(state.db) else {
                        unreachable!()
                    };
                    *id == n && i.to_ty(state.db).is_none()
                })
                .flat_map(|i| {
                    let mut funcs = Vec::new();
                    let mut implied = HashMap::new();
                    i.from_ty(state.db).imply_generic_args(self, &mut implied);
                    let self_ty = self.parameterize(&implied);
                    implied.insert("Self".to_string(), self_ty);
                    for func in i.functions(state.db) {
                        if func.name(state.db) != name.0 {
                            continue;
                        }
                        let ty = func.get_ty(state).parameterize(&implied);
                        if let Ty::Function(ty) = ty {
                            funcs.push((IdentDef::Decl(func), ty));
                        }
                    }
                    funcs
                })
                .collect::<Vec<_>>();
            impl_funcs.pop()
        } else {
            None
        }
    }

    pub fn get_funcs(&self, state: &mut CheckState<'db>) -> Vec<(Decl<'db>, FuncTy<'db>)> {
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

pub fn path_to_sub_ty<'db>(
    name: ModulePath<'db>,
    sub_ty: ModulePath<'db>,
    state: &mut CheckState<'db>,
) -> Option<Vec<ImplForDecl<'db>>> {
    if name == sub_ty {
        return Some(Vec::new());
    }
    let (id, mut path) = state
        .project
        .get_impls(state.db, name)
        .into_iter()
        .filter(|i| i.to_ty(state.db).is_some())
        .map(|i| {
            if let Ty::Named(Named { name, .. }) = i.to_ty(state.db).unwrap() {
                (i, name)
            } else {
                unreachable!()
            }
        })
        .find_map(|(id, n)| path_to_sub_ty(n, sub_ty, state).map(|p| (id, p)))?;
    path.insert(0, id);
    Some(path)
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

fn check_named_is_instance_of_named<'db>(
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
                        if !arg.check_is_instance_of(other, state, span)
                            || !other.check_is_instance_of(&arg, state, span)
                        {
                            return false;
                        }
                    }
                    Variance::Covariant => {
                        if !arg.check_is_instance_of(other, state, span) {
                            return false;
                        }
                    }
                    Variance::Contravariant => {
                        if !other.check_is_instance_of(&arg, state, span) {
                            return false;
                        }
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
