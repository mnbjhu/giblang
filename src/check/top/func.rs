use crate::{
    check::{
        expr::code_block::{check_code_block, check_code_block_is},
        state::CheckState,
    },
    parser::top::func::Func,
    project::decl::Function,
    ty::Ty,
};

impl<'db> Func {
    pub fn check(&self, state: &mut CheckState<'_>, allow_empty: bool) {
        self.generics.0.check(state);
        if let Some(rec) = &self.receiver {
            rec.0.check(state);
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

    pub fn check_matches(&self, trait_decl: &Function<'db>, state: &mut CheckState<'db>) {
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
                .zip(&trait_decl.generics)
                .for_each(|((i, s), t)| {
                    if i != t {
                        state.simple_error(
                            &format!(
                                "Expected '{}' but found '{}'",
                                t.get_name(state),
                                i.get_name(state)
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

        let spanned_decl_args = args.iter().zip(self.args.iter().map(|(_, s)| s));

        spanned_decl_args
            .zip(&trait_decl.args)
            .for_each(|((i, s), t)| {
                i.1.expect_is_instance_of(&t.1, state, false, *s);
            });

        let ret = self
            .ret
            .as_ref()
            .map_or(Ty::unit(), |(ret, _)| ret.check(state));

        ret.expect_is_instance_of(&trait_decl.ret, state, false, self.name.1);

        let receiver = self.receiver.as_ref().map(|(rec, _)| rec.check(state));

        match (&receiver, &trait_decl.receiver) {
            (Some(i), Some(t)) => {
                i.expect_is_instance_of(t, state, false, self.name.1);
            }
            (None, None) => {}
            (None, Some(ty)) => {
                state.simple_error(
                    &format!(
                        "Expected receiver of type '{}' but found none",
                        ty.get_name(state)
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
