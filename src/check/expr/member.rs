use std::collections::HashMap;

use crate::{
    check::state::CheckState,
    parser::expr::{member::MemberCall, Expr},
    project::Project,
    ty::Ty,
    util::Span,
};

use super::ident::check_ident;

impl MemberCall {
    pub fn check<'module>(
        &'module self,
        project: &'module Project,
        state: &mut CheckState<'module>,
    ) -> Ty {
        let ty = check_ident(state, &vec![self.name.clone()], project);

        if let Ty::Function {
            args: expected_args,
            ret,
            receiver: Some(receiver),
        } = &ty
        {
            let mut generics = ty.get_generic_params();
            let mut implied = HashMap::<String, Ty>::new();

            imply_generic(
                self.rec.0.as_ref(),
                receiver,
                project,
                state,
                self.rec.1,
                &mut implied,
            );

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
                    imply_generic(arg, expected, project, state, *span, &mut implied);
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
                state.simple_error(
                    &format!("Couldn't imply generic ty args: {}", not_implied),
                    self.name.1,
                )
            }

            ret.as_ref().parameterize(&implied)
        } else {
            Ty::Unknown
        }
    }

    pub fn expected_instance_of<'module>(
        &'module self,
        expected: &Ty,
        project: &'module Project,
        state: &mut CheckState<'module>,
        span: Span,
    ) -> Ty {
        let actual = self.check(project, state);
        if !actual.is_instance_of(expected, project) {
            state.simple_error(
                &format!("Expected value to be of type '{expected}' but found '{actual}'",),
                span,
            )
        }
        actual
    }
}

fn imply_generic<'module>(
    actual: &'module Expr,
    expected: &Ty,
    project: &'module Project,
    state: &mut CheckState<'module>,
    span: chumsky::prelude::SimpleSpan,
    implied: &mut HashMap<String, Ty>,
) {
    let actual = actual.expect_instance_of(expected, project, state, span);
    let implied_geneircs = expected.imply_generics(&actual);
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
}
