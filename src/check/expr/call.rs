use std::collections::HashMap;

use crate::{
    check::state::CheckState, fs::project::Project, parser::expr::call::Call, ty::Ty, util::Span,
};

impl Call {
    pub fn check<'module>(
        &'module self,
        project: &'module Project,
        state: &mut CheckState<'module>,
    ) -> Ty<'module> {
        let name_ty = self.name.0.check(project, state);
        // TODO: Think about receivers
        if let Ty::Function {
            args: expected_args,
            ret,
            ..
        } = &name_ty
        {
            let mut generics = name_ty.get_generic_params();
            if expected_args.len() != self.args.len() {
                state.error(
                    &format!(
                        "Expected {} arguments but found {}",
                        expected_args.len(),
                        self.args.len()
                    ),
                    self.name.1,
                );
            }

            let mut implied = HashMap::<String, Ty<'_>>::new();

            self.args
                .iter()
                .zip(expected_args)
                .for_each(|((arg, span), expected)| {
                    let actual = arg.expect_instance_of(expected, project, state, *span);
                    let implied_geneircs = expected.imply_generics(actual);
                    if let Some(implied_geneircs) = implied_geneircs {
                        for (name, ty) in implied_geneircs {
                            let new = if let Some(existing) = implied.get(&name) {
                                existing.get_shared_subtype(&ty, project)
                            } else {
                                ty
                            };
                            implied.insert(name, new);
                        }
                    }
                });

            generics.retain(|g| !implied.contains_key(&g.name));
            for g in &generics {
                implied.insert(g.name.clone(), Ty::Unknown);
            }

            if !generics.is_empty() {
                let not_implied = generics
                    .iter()
                    .cloned()
                    .map(|g| g.name)
                    .collect::<Vec<_>>()
                    .join(", ");
                state.error(
                    &format!("Couldn't imply generic ty args: {}", not_implied),
                    self.name.1,
                )
            }

            ret.as_ref().parameterize(&implied)
        } else if let Ty::Unknown = name_ty {
            Ty::Unknown
        } else {
            state.error(
                &format!("Expected a function but found '{name_ty}'"),
                self.name.1,
            );
            Ty::Unknown
        }
    }

    pub fn expected_instance_of<'module>(
        &'module self,
        expected: &Ty<'module>,
        project: &'module Project,
        state: &mut CheckState<'module>,
        span: Span,
    ) -> Ty<'module> {
        let actual = self.check(project, state);
        if !actual.is_instance_of(expected, project) {
            state.error(
                &format!("Expected value to be of type '{expected}' but found '{actual}'",),
                span,
            )
        }
        actual
    }
}
