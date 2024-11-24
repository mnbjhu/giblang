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
            scopes: vec![Scope::default()],
        }
    }
}

impl<'db> Scoped<'db> for ScopedState<'db> {
    fn enter_scope(&mut self) {
        self.scopes.push(Scope::default());
    }
    #[must_use]
    fn exit_scope(&mut self) -> Scope<'db> {
        self.scopes.pop().expect("Scope stack underflow")
    }

    fn get_variable(&self, name: &str) -> Option<&VarDecl<'db>> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.vars.get(name))
    }

    fn get_variables(&self) -> HashMap<&String, &VarDecl<'db>> {
        let mut found = HashMap::new();
        for scope in self.scopes.iter().rev() {
            for (name, var) in &scope.vars {
                found.insert(name, var);
            }
        }
        found
    }

    fn get_generic(&self, name: &str) -> Option<&Generic<'db>> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.generics.get(name))
    }

    fn get_generics(&self) -> HashMap<&String, &Generic<'db>> {
        let mut found = HashMap::new();
        for scope in self.scopes.iter().rev() {
            for (name, generic) in &scope.generics {
                found.insert(name, generic);
            }
        }
        found
    }

    fn get_import(&self, name: &str) -> Option<Decl<'db>> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.imports.get(name).copied())
    }

    fn get_imports(&self) -> HashMap<&String, &Decl<'db>> {
        let mut found = HashMap::new();
        for scope in self.scopes.iter().rev() {
            for (name, decl) in &scope.imports {
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
        self.scope_mut().vars.insert(name.to_string(), var);
    }

    fn insert_generic(&mut self, name: &str, generic: Generic<'db>) {
        self.scope_mut().generics.insert(name.to_string(), generic);
    }

    fn insert_import(&mut self, name: &str, import: Decl<'db>) {
        self.scope_mut().imports.insert(name.to_string(), import);
    }
}

#[derive(Default, PartialEq, Debug, Clone)]
pub struct Scope<'db> {
    pub imports: HashMap<String, Decl<'db>>,
    pub vars: HashMap<String, VarDecl<'db>>,
    pub generics: HashMap<String, Generic<'db>>,
}
