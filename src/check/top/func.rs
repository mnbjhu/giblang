use std::{collections::HashMap, ops::ControlFlow};

use crate::{
    check::{state::CheckState, ControlIter, Dir},
    db::decl::func::Function,
    item::AstItem,
    parser::top::func::Func,
    ty::{Generic, Ty},
    util::{Span, Spanned},
};

use super::Check;

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
                    &format!("Expected receiver of type '{}'", ty.get_name(state, None)),
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

impl<'ast, 'db, Iter: ControlIter<'ast>> Check<'ast, 'db, Iter, (), bool> for Spanned<Func> {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: Span,
        allow_empty: bool,
    ) -> ControlFlow<&'ast dyn AstItem> {
        control.act(&self.0, state, Dir::Enter, self.1)?;
        self.0.generics.0.check(state);
        if let Some(rec) = &self.0.receiver {
            let self_ty = rec.0.check(state);
            self.0.add_self_param(self_ty, state);
        }
        for arg in &self.0.args {
            arg.0.check(state);
        }
        if self.0.ret.is_some() && !allow_empty {
            let expected = self.0.ret.as_ref().unwrap().0.check(state);
            self.0
                .body
                .as_ref()
                .unwrap_or(&vec![])
                .expect(state, control, &expected, span, ());
        } else if let Some(body) = &self.0.body {
            body.check(state, control, span, ());
        }
        control.act(&self.0, state, Dir::Exit, self.1)?;
        ControlFlow::Continue(())
    }
}
