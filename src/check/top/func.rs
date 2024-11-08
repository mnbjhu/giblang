use std::{collections::HashMap, ops::ControlFlow};

use crate::{
    check::{state::CheckState, ControlIter, Dir},
    db::decl::func::Function,
    item::AstItem,
    parser::top::func::Func,
    ty::{Generic, Ty},
    util::Span,
};

use super::Check;

impl<'ast, 'db> Func {
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

    #[allow(clippy::too_many_lines)]
    #[must_use]
    pub fn check_matches<Iter: ControlIter<'ast, 'db>>(
        &'ast self,
        trait_decl: &Function<'db>,
        state: &mut CheckState<'db>,
        // FROM THE TRAIT
        params: &HashMap<String, Ty<'db>>,
        control: &mut Iter,
        span: Span,
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>)> {
        control.act(self, state, Dir::Enter, span)?;
        let generics = self.generics.0.check(state, control, self.generics.1, ())?;
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

        let receiver = if let Some(r) = &self.receiver {
            let ty = r.0.check(state, control, r.1, ())?;
            self.add_self_param(ty.clone(), state);
            Some(ty)
        } else {
            None
        };

        let mut args = Vec::new();
        for arg in &self.args {
            args.push(arg.0.check(state, control, arg.1, ())?);
        }

        let spanned_decl_args = args.iter().zip(self.args.iter().map(|(arg, _)| arg.ty.1));

        spanned_decl_args
            .zip(trait_decl.args.iter().map(|arg| arg.1.parameterize(params)))
            .for_each(|((i, s), t)| {
                i.1.expect_is_instance_of(&t, state, s);
            });

        let ret_ty = if let Some((ret, span)) = &self.ret {
            ret.check(state, control, *span, ())?
        } else {
            Ty::unit()
        };

        let ret = if let Some(ret) = &self.ret {
            ret_ty.expect_is_instance_of(&trait_decl.ret.parameterize(params), state, ret.1);
            ret_ty
        } else if !trait_decl.ret.is_unit() {
            state.simple_error(
                &format!(
                    "Expected return type of '{}' but found none",
                    trait_decl.ret.get_name(state, None)
                ),
                self.name.1,
            );
            Ty::unit()
        } else {
            Ty::Unknown
        };

        match (
            &receiver,
            trait_decl.receiver.as_ref().map(|r| r.parameterize(params)),
        ) {
            (Some(i), Some(t)) => {
                i.expect_is_instance_of(&t, state, self.name.1);
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
        let body_span = if let Some(body) = &self.body {
            if body.is_empty() {
                self.name.1
            } else {
                Span::new(body[0].1.start, body.last().unwrap().1.end)
            }
        } else {
            self.name.1
        };
        if let Some(body) = &self.body {
            body.expect(state, control, &ret, body_span, ())?;
        }
        control.act(self, state, Dir::Exit(Ty::unit()), span)?;
        ControlFlow::Continue(())
    }
}

impl<'ast, 'db, Iter: ControlIter<'ast, 'db>> Check<'ast, 'db, Iter, (), bool> for Func {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: Span,
        allow_empty: bool,
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), ()> {
        control.act(self, state, Dir::Enter, span)?;
        self.generics.0.check(state, control, self.generics.1, ())?;
        if let Some(rec) = &self.receiver {
            let self_ty = rec.0.check(state, control, rec.1, ())?;
            self.add_self_param(self_ty, state);
        }
        for arg in &self.args {
            arg.0.check(state, control, arg.1, ())?;
        }
        let expected = if let Some(ret) = &self.ret {
            ret.0.check(state, control, ret.1, ())?
        } else {
            Ty::unit()
        };
        if !allow_empty || self.body.is_some() {
            if expected.is_unit() {
                if let Some(body) = &self.body {
                    body.check(state, control, span, ())?;
                }
            } else if let Some(body) = &self.body {
                body.expect(state, control, &expected, span, ())?;
            } else {
                Ty::unit().expect_is_instance_of(&expected, state, self.name.1);
            }
        } else if let Some(body) = &self.body {
            body.check(state, control, span, ())?;
        }
        control.act(self, state, Dir::Exit(Ty::unit()), span)?;
        ControlFlow::Continue(())
    }
}
