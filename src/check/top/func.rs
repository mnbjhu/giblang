use std::collections::HashMap;

use crate::{
    check::{
        expr::code_block::{check_code_block, check_code_block_is},
        state::CheckState,
    },
    db::decl::func::Function,
    parser::top::func::Func,
    ty::{Generic, Ty},
};

impl<'db> Func {
    pub fn add_self_param(&self, self_ty: Ty<'db>, state: &mut CheckState<'db>) {
        match self_ty {
            Ty::Generic(Generic { name, super_, .. }) if name.0 == "Self" => {
                state.add_self_param(super_.as_ref().clone(), self.receiver.as_ref().unwrap().1);
            }
            _ => {
                state.add_self_param(self_ty.clone(), self.receiver.as_ref().unwrap().1);
            }
        }
    }

    pub fn check(&self, state: &mut CheckState<'_>, allow_empty: bool) {
        self.generics.0.check(state);
        if let Some(rec) = &self.receiver {
            let self_ty = rec.0.check(state);
            self.add_self_param(self_ty, state);
        }
        for arg in &self.args {
            arg.0.check(state);
        }
        if self.ret.is_some() && !allow_empty {
            let expected = self.ret.as_ref().unwrap().0.check(state);
            check_code_block_is(
                state,
                &expected,
                self.body.as_ref().unwrap_or(&vec![]),
                self.name.1,
            );
        } else if let Some(body) = &self.body {
            check_code_block(state, body);
        }
    }

    pub fn check_matches(
        &self,
        trait_decl: &Function<'db>,
        state: &mut CheckState<'db>,
        // FROM THE TRAIT
        params: &HashMap<String, Ty<'db>>,
    ) {
        let generics = self.generics.0.check(state);
        let spanned_decl_generics = generics
            .iter()
            .map(|g| {
                let Ty::Generic(g) = g else {
                    panic!("Expected generic type but found {g:?}");
                };
                g
            })
            .zip(self.generics.0 .0.iter().map(|(_, s)| s));

        if trait_decl.generics.len() == generics.len() {
            spanned_decl_generics
                .zip(trait_decl.generics.iter().map(|g| g.parameterize(params)))
                .for_each(|((i, s), t)| {
                    if i != &t {
                        state.simple_error(
                            &format!(
                                "Expected '{}' but found '{}'",
                                t.get_name(state, None),
                                i.get_name(state, None)
                            ),
                            *s,
                        );
                    }
                });
        } else {
            state.simple_error(
                &format!(
                    "Expected {} generic parameters but found {}",
                    trait_decl.generics.len(),
                    generics.len()
                ),
                self.generics.1,
            );
        }

        let args = self
            .args
            .iter()
            .map(|arg| arg.0.check(state))
            .collect::<Vec<_>>();

        let spanned_decl_args = args.iter().zip(self.args.iter().map(|(arg, _)| arg.ty.1));

        spanned_decl_args
            .zip(trait_decl.args.iter().map(|arg| arg.1.parameterize(params)))
            .for_each(|((i, s), t)| {
                i.1.expect_is_instance_of(&t, state, false, s);
            });

        let ret_ty = self
            .ret
            .as_ref()
            .map_or(Ty::unit(), |(ret, _)| ret.check(state));

        if let Some(ret) = &self.ret {
            ret_ty.expect_is_instance_of(&trait_decl.ret.parameterize(params), state, false, ret.1);
        } else if !trait_decl.ret.is_unit() {
            state.simple_error(
                &format!(
                    "Expected return type of '{}' but found none",
                    trait_decl.ret.get_name(state, None)
                ),
                self.name.1,
            );
        };

        let receiver = self.receiver.as_ref().map(|(rec, _)| rec.check(state));

        match (
            &receiver,
            trait_decl.receiver.as_ref().map(|r| r.parameterize(params)),
        ) {
            (Some(i), Some(t)) => {
                i.expect_is_instance_of(&t, state, false, self.name.1);
            }
            (None, None) => {}
            (None, Some(ty)) => {
                state.simple_error(
                    &format!(
                        "Expected receiver of type '{}'",
                        ty.get_name(state, None)
                    ),
                    self.name.1,
                );
            }
            (Some(_), None) => {
                state.simple_error(
                    "Expected no receiver but found one",
                    self.receiver.as_ref().unwrap().1,
                );
            }
        }
    }
}
