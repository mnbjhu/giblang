use std::{collections::HashMap, ops::ControlFlow};

use crate::{
    check::{state::CheckState, ControlIter, Dir},
    db::decl::DeclKind,
    item::AstItem,
    parser::top::impl_::Impl,
    ty::Ty,
    util::{Span, Spanned},
};

use super::Check;

impl<'ast, 'db, Iter: ControlIter<'ast>> Check<'ast, 'db, Iter> for Spanned<Impl> {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: Span,
        (): (),
    ) -> ControlFlow<&'ast dyn AstItem> {
        control.act(&self.0, state, Dir::Exit, self.1)?;
        self.0.generics.0.check(state);
        let for_ = self.0.for_.0.check(state);
        state.add_self_ty(&for_, self.0.for_.1);
        if let Some(trait_) = &self.0.trait_ {
            let trait_ty = trait_.0.check(state);
            if let Ty::Named { name, .. } = &trait_ty {
                if let DeclKind::Trait { body, generics, .. } = state
                    .project
                    .get_decl(state.db, *name)
                    .unwrap()
                    .kind(state.db)
                {
                    let trait_decl_ty = Ty::Named {
                        name: *name,
                        args: generics.iter().map(|g| Ty::Generic(g.clone())).collect(),
                    };
                    let mut params = HashMap::new();
                    trait_decl_ty.imply_generic_args(&trait_ty, &mut params);
                    let mut found = Vec::new();
                    for func in &self.0.body {
                        let expected = body.iter().find_map(|mod_| {
                            if mod_.name(state.db) == func.0.name.0 {
                                Some(mod_.into_func(state.db))
                            } else {
                                None
                            }
                        });
                        if let Some(expected) = expected {
                            found.push(expected);
                            func.0.check_matches(expected, state, &params);
                        } else {
                            state.simple_error(
                                &format!(
                                    "No function with name '{}' found on trait {}",
                                    func.0.name.0,
                                    name.name(state.db).join("::")
                                ),
                                func.0.name.1,
                            );
                        }
                    }
                    let mut missing = Vec::new();
                    for mod_ in body {
                        let func = mod_.into_func(state.db);
                        if !found.contains(&func) && func.required {
                            missing.push(func);
                        }
                    }
                    if !missing.is_empty() {
                        state.simple_error(
                            &format!(
                                "Missing implementations for functions: {}",
                                missing
                                    .iter()
                                    .map(|f| f.name.to_string())
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            ),
                            self.0.for_.1,
                        );
                    }
                } else {
                    state.simple_error(
                        &format!("Expected trait, found {}", name.name(state.db).join("::")),
                        trait_.0.name.last().unwrap().1,
                    );
                };
            }
        } else {
            for func in &self.0.body {
                state.enter_scope();
                func.check(state, control, span, false)?;
                state.exit_scope();
            }
        }
        control.act(&self.0, state, Dir::Exit, self.1)?;
        ControlFlow::Continue(())
    }
}
