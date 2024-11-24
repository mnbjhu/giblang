use std::collections::HashMap;

use crate::{
    db::{
        decl::{Decl, Project},
        input::{Db, SourceFile},
    },
    ty::{Generic, Ty},
    util::Span,
};

use super::{
    scoped_state::{Scope, Scoped, ScopedState},
    state::VarDecl,
};

pub trait IsScoped<'db> {
    fn get_scope_state<'me>(&'me self) -> &'me ScopedState<'db>;
    fn get_scope_state_mut<'me>(&'me mut self) -> &'me mut ScopedState<'db>;
    fn get_type_var(&self, id: u32) -> Ty<'db>;
    fn expected_type_var_is(&mut self, id: u32, other: Ty<'db>, span: Span);
}

impl<'db, T: IsScoped<'db>> Scoped<'db> for T {
    fn push_scope(&mut self, scope: Scope<'db>) {
        self.get_scope_state_mut().push_scope(scope);
    }
    fn get_variable(&self, name: &str) -> Option<&VarDecl<'db>> {
        self.get_scope_state().get_variable(name)
    }

    fn get_variables(&self) -> HashMap<&String, &VarDecl<'db>> {
        self.get_scope_state().get_variables()
    }

    fn insert_variable(&mut self, name: &str, var: VarDecl<'db>) {
        self.get_scope_state_mut().insert_variable(name, var);
    }
    fn insert_generic(&mut self, name: &str, g: Generic<'db>) {
        self.get_scope_state_mut().insert_generic(name, g);
    }
    fn insert_import(&mut self, name: &str, import: Decl<'db>) {
        self.get_scope_state_mut().insert_import(name, import);
    }

    fn get_generic(&self, name: &str) -> Option<&Generic<'db>> {
        self.get_scope_state().get_generic(name)
    }

    fn get_generics(&self) -> HashMap<&String, &Generic<'db>> {
        self.get_scope_state().get_generics()
    }

    fn get_import(&self, name: &str) -> Option<Decl<'db>> {
        self.get_scope_state().get_import(name)
    }

    fn get_imports(&self) -> HashMap<&String, &Decl<'db>> {
        self.get_scope_state().get_imports()
    }

    fn enter_scope(&mut self) {
        self.get_scope_state_mut().enter_scope();
    }

    fn exit_scope(&mut self) -> Scope<'db> {
        self.get_scope_state_mut().exit_scope()
    }

    fn project(&self) -> Project<'db> {
        self.get_scope_state().project()
    }

    fn get_file(&self) -> SourceFile {
        self.get_scope_state().get_file()
    }

    fn db(&self) -> &'db dyn Db {
        self.get_scope_state().db()
    }
    fn order(&self) -> usize {
        self.get_scope_state().order()
    }
    fn inc_order(&mut self) -> usize {
        self.get_scope_state_mut().inc_order()
    }
    fn set_order(&mut self, order: usize) {
        self.get_scope_state_mut().set_order(order);
    }
}
