use std::collections::HashMap;

use crate::{
    db::{
        decl::{Decl, Project},
        input::{Db, SourceFile},
        path::ModulePath,
    },
    ir::common::pattern::SpannedQualifiedNameIR,
    item::definitions::ident::IdentDef,
    ty::Generic,
    util::Spanned,
};

use super::{err::unresolved::Unresolved, state::VarDecl};

pub trait Scoped<'db>: Sized {
    fn project(&self) -> Project<'db>;
    fn get_file(&self) -> SourceFile;
    fn db(&self) -> &'db dyn Db;

    fn push_scope(&mut self, scope: Scope<'db>);

    fn get_variable(&self, name: &str) -> Option<&VarDecl<'db>>;
    fn get_variables(&self) -> HashMap<&String, &VarDecl<'db>>;
    fn insert_variable(&mut self, name: &str, var: VarDecl<'db>);

    fn get_generic(&self, name: &str) -> Option<&Generic<'db>>;
    fn get_generics(&self) -> HashMap<&String, &Generic<'db>>;
    fn insert_generic(&mut self, name: &str, g: Generic<'db>);

    fn get_import(&self, name: &str) -> Option<Decl<'db>>;
    fn get_imports(&self) -> HashMap<&String, &Decl<'db>>;
    fn insert_import(&mut self, name: &str, var: Decl<'db>);

    fn enter_scope(&mut self);
    fn exit_scope(&mut self) -> Scope<'db>;

    fn order(&self) -> usize;
    fn inc_order(&mut self) -> usize;
    fn set_order(&mut self, order: usize);

    fn get_decl_with_error(&self, path: &[Spanned<String>]) -> Result<Decl<'db>, Unresolved>
    where
        Self: Sized,
    {
        if let Some(import) = self.get_import(&path[0].0) {
            import.try_get_path(self, &path[1..])
        } else {
            self.project().decls(self.db()).try_get_path(self, path)
        }
    }

    fn get_ident_ir(&mut self, path: &[Spanned<String>]) -> SpannedQualifiedNameIR<'db> {
        if let Some(import) = self.get_import(&path[0].0) {
            let mut found = import.get_path_ir(self, &path[1..]);
            found.insert(0, (IdentDef::Decl(import), path[0].1));
            found
        } else {
            self.project().decls(self.db()).get_path_ir(self, path)
        }
    }

    fn try_get_decl_path(&self, name: ModulePath<'db>) -> Option<Decl<'db>> {
        self.project().get_decl(self.db(), name)
    }
}

pub struct ScopedState<'db> {
    pub db: &'db dyn Db,
    pub file: SourceFile,
    pub project: Project<'db>,
    pub scopes: Vec<Scope<'db>>,
}

impl<'db> ScopedState<'db> {
    pub fn scope(&self) -> &Scope<'db> {
        self.scopes.last().unwrap()
    }
    pub fn scope_mut(&mut self) -> &mut Scope<'db> {
        self.scopes.last_mut().unwrap()
    }
}

impl<'db> ScopedState<'db> {
    pub fn new(db: &'db dyn Db, project: Project<'db>, file: SourceFile) -> Self {
        Self {
            db,
            file,
            project,
            scopes: vec![Scope::new()],
        }
    }
}

impl<'db> Scoped<'db> for ScopedState<'db> {
    fn enter_scope(&mut self) {
        self.scopes.push(Scope::new());
    }
    #[must_use]
    fn exit_scope(&mut self) -> Scope<'db> {
        self.scopes.pop().expect("Scope stack underflow")
    }

    fn get_variable(&self, name: &str) -> Option<&VarDecl<'db>> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.vars.get(name, scope.order))
    }

    fn get_variables(&self) -> HashMap<&String, &VarDecl<'db>> {
        let mut found = HashMap::new();
        for scope in self.scopes.iter().rev() {
            for (name, item) in scope.vars.get_all(scope.order) {
                found.insert(name, item);
            }
        }
        found
    }

    fn get_generic(&self, name: &str) -> Option<&Generic<'db>> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.generics.get(name, scope.order))
    }

    fn get_generics(&self) -> HashMap<&String, &Generic<'db>> {
        let mut found = HashMap::new();
        for scope in self.scopes.iter().rev() {
            for (name, generic) in scope.generics.get_all(scope.order) {
                found.insert(name, generic);
            }
        }
        found
    }

    fn get_import(&self, name: &str) -> Option<Decl<'db>> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.imports.get(name, scope.order).copied())
    }

    fn get_imports(&self) -> HashMap<&String, &Decl<'db>> {
        let mut found = HashMap::new();
        for scope in self.scopes.iter().rev() {
            for (name, decl) in scope.imports.get_all(scope.order) {
                found.insert(name, decl);
            }
        }
        found
    }

    fn project(&self) -> Project<'db> {
        self.project
    }

    fn get_file(&self) -> SourceFile {
        self.file
    }

    fn db(&self) -> &'db dyn Db {
        self.db
    }

    fn push_scope(&mut self, scope: Scope<'db>) {
        self.scopes.push(scope);
    }

    fn insert_variable(&mut self, name: &str, var: VarDecl<'db>) {
        let order = self.order();
        self.scope_mut().vars.insert(name.to_string(), var, order);
    }

    fn insert_generic(&mut self, name: &str, generic: Generic<'db>) {
        let order = self.order();
        self.scope_mut()
            .generics
            .insert(name.to_string(), generic, order);
    }

    fn insert_import(&mut self, name: &str, import: Decl<'db>) {
        let order = self.order();
        self.scope_mut()
            .imports
            .insert(name.to_string(), import, order);
    }

    fn order(&self) -> usize {
        self.scope().order
    }

    fn inc_order(&mut self) -> usize {
        let order = &mut self.scope_mut().order;
        *order += 1;
        *order
    }

    fn set_order(&mut self, order: usize) {
        self.scope_mut().order = order;
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Scope<'db> {
    pub imports: OrderedMap<Decl<'db>>,
    pub vars: OrderedMap<VarDecl<'db>>,
    pub generics: OrderedMap<Generic<'db>>,
    pub order: usize,
}

impl<'db> Scope<'db> {
    pub fn new() -> Self {
        Self {
            imports: OrderedMap::new(),
            vars: OrderedMap::new(),
            generics: OrderedMap::new(),
            order: 0,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct OrderedMap<T> {
    items: Vec<OrderMapItem<T>>,
}

impl<T> OrderedMap<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
}

#[derive(Default, PartialEq, Debug, Clone)]
struct OrderMapItem<T> {
    order: usize,
    name: String,
    value: T,
}

impl<T> OrderedMap<T> {
    pub fn insert(&mut self, name: String, value: T, order: usize) {
        self.items.push(OrderMapItem { order, name, value });
    }

    pub fn get(&self, name: &str, order: usize) -> Option<&T> {
        let mut found = None;
        for item in &self.items {
            if order < item.order {
                break;
            }
            if name == item.name {
                found = Some(&item.value);
            }
        }
        found
    }

    pub fn get_all(&self, order: usize) -> Vec<(&String, &T)> {
        let mut found = vec![];
        for item in &self.items {
            if order < item.order {
                break;
            }
            found.push((&item.name, &item.value));
        }
        found
    }
}
