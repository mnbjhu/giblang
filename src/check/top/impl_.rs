use std::{collections::HashMap, ops::ControlFlow};

use crate::{
    check::{state::CheckState, ControlIter, Dir},
    db::decl::DeclKind,
    item::AstItem,
    parser::top::impl_::Impl,
    ty::Ty,
    util::Span,
};

use super::Check;

impl<'ast, 'db, Iter: ControlIter<'ast, 'db>> Check<'ast, 'db, Iter, ()> for Impl {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: Span,
        (): (),
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), ()> {
        control.act(self, state, Dir::Enter, span)?;
        self.generics.0.check(state, control, self.generics.1, ())?;
        let for_ = self.for_.0.check(state, control, self.for_.1, ())?;
        state.add_self_ty(&for_, self.for_.1);
        if let Some(trait_) = &self.trait_ {
            let trait_ty = trait_.0.check(state, control, trait_.1, ())?;
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
                    for (func, span) in &self.body {
                        let expected = body.iter().find_map(|decl| {
                            if decl.name(state.db) == func.name.0 {
                                Some(decl.into_func(state.db))
                            } else {
                                None
                            }
                        });
                        if let Some(expected) = expected {
                            found.push(expected);
                            func.check_matches(expected, state, &params, control, *span)?;
                        } else {
                            func.check(state, control, *span, true)?;
                            state.simple_error(
                                &format!(
                                    "No function with name '{}' found on trait {}",
                                    func.name.0,
                                    name.name(state.db).join("::")
                                ),
                                func.name.1,
                            );
                        }
                    }
                    let mut missing = Vec::new();
                    for decl in body {
                        let func = decl.into_func(state.db);
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
                            self.for_.1,
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
            for (func, span) in &self.body {
                state.enter_scope();
                func.check(state, control, *span, false)?;
                state.exit_scope();
            }
        }
        control.act(self, state, Dir::Exit(Ty::unit()), span)?;
        ControlFlow::Continue(())
    }
}
