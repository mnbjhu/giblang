use crate::{
    check::state::CheckState,
    db::{
        input::Db,
        modules::{Module, ModuleData},
    },
    parser::top::impl_::Impl,
    project::decl::{Decl, DeclKind, Function},
    ty::Ty,
};

impl<'db> Impl {
    pub fn check(&'db self, state: &mut CheckState<'db>) {
        self.generics.0.check(state);
        let for_ = self.for_.0.check(state);
        state.add_self_ty(for_, self.for_.1);
        if let Some(trait_) = &self.trait_ {
            if let Ty::Named { name, .. } = trait_.0.check(state) {
                if let DeclKind::Trait { body, .. } = state
                    .project
                    .get_decl(state.db, name)
                    .unwrap()
                    .kind(state.db)
                {
                    let mut found = Vec::new();
                    for func in &self.body {
                        let expected = body.iter().find_map(|mod_| {
                            if mod_.name(state.db) == func.0.name.0 {
                                Some(mod_.into_func(state.db))
                            } else {
                                None
                            }
                        });
                        if let Some(expected) = expected {
                            found.push(expected);
                            func.0.check_matches(expected, state);
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
            for func in &self.body {
                state.enter_scope();
                func.0.check(state, false);
                state.exit_scope();
            }
        }
    }
}

impl<'db> Module<'db> {
    pub fn into_func(self, db: &'db dyn Db) -> &'db Function<'db> {
        if let ModuleData::Export(e) = self.content(db) {
            e.into_func(db)
        } else {
            panic!("Expected export");
        }
    }
}

impl<'db> Decl<'db> {
    pub fn into_func(self, db: &'db dyn Db) -> &'db Function<'db> {
        if let DeclKind::Function(f) = self.kind(db) {
            f
        } else {
            panic!("Expected function");
        }
    }
}
