use std::collections::HashMap;

use crate::{
    check::state::CheckState,
    parser::expr::member::MemberCall,
    project::decl::{struct_::StructDecl, DeclKind},
    ty::{is_instance::get_sub_tys, FuncTy, Ty},
    util::{Span, Spanned},
};

use super::ident::check_ident;

impl<'db> MemberCall {
    pub fn check(&self, state: &mut CheckState<'db>) -> Ty<'db> {
        let rec = self.rec.0.check(state);
        let Some(func_ty) = rec.get_member_func(&self.name, state) else {
            state.simple_error(
                &format!(
                    "No function {} found for type {}",
                    self.name.0,
                    rec.get_name(state)
                ),
                self.name.1,
            );
            return Ty::Unknown;
        };
        let FuncTy {
            args: expected_args,
            ret,
            receiver,
        } = func_ty;
        if let Some(rec) = receiver {
            self.rec.0.expect_instance_of(&rec, state, self.rec.1);
        }

        if expected_args.len() != self.args.len() {
            state.simple_error(
                &format!(
                    "Expected {} arguments but found {}",
                    expected_args.len(),
                    self.args.len()
                ),
                self.name.1,
            );
        }

        self.args
            .iter()
            .zip(expected_args)
            .for_each(|((arg, span), expected)| {
                arg.expect_instance_of(&expected, state, *span);
            });
        ret.as_ref().clone()
    }

    pub fn expected_instance_of(
        &self,
        expected: &Ty<'db>,
        state: &mut CheckState<'db>,
        span: Span,
    ) {
        let actual = self.check(state);
        actual.expect_is_instance_of(expected, state, false, span);
    }
}

impl<'db> Ty<'db> {
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
