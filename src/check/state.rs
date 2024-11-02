use std::{collections::HashMap, vec};

use salsa::Accumulator;

use crate::{
    check::err::{simple::Simple, CheckError},
    db::{
        decl::{Decl, Project},
        input::{Db, SourceFile},
        path::ModulePath,
    },
    parser::{
        expr::qualified_name::{QualifiedName, SpannedQualifiedName},
        parse_file,
    },
    ty::{Generic, Ty},
    util::{Span, Spanned},
};

use super::{
    err::{unresolved::Unresolved, unresolved_type_var::UnboundTypeVar, Error, IntoWithDb},
    type_state::TypeState, TokenKind,
};

#[derive(Debug, Clone)]
pub struct VarDecl<'db> {
    pub name: String,
    pub ty: Ty<'db>,
    pub kind: TokenKind,
    pub span: Span,
}

pub struct CheckState<'db> {
    pub db: &'db dyn Db,
    imports: HashMap<String, ModulePath<'db>>,
    generics: Vec<HashMap<String, Generic<'db>>>,
    variables: Vec<HashMap<String, VarDecl<'db>>>,
    pub file_data: SourceFile,
    pub project: Project<'db>,
    pub type_state: TypeState<'db>,
    pub path: Vec<String>,
    pub should_error: bool,
}

impl<'ty, 'db: 'ty> CheckState<'db> {
    pub fn from_file(
        db: &'db dyn Db,
        file_data: SourceFile,
        project: Project<'db>,
    ) -> CheckState<'db> {
        let mut state = CheckState {
            db,
            imports: HashMap::new(),
            generics: vec![],
            variables: vec![],
            file_data,
            project,
            type_state: TypeState::default(),
            path: file_data.module_path(db).name(db).clone(),
            should_error: true,
        };
        let mut path = file_data.module_path(db).name(db).clone();
        let tops = parse_file(db, file_data).tops(db);
        for top in tops {
            if let Some(name) = top.0.get_name() {
                path.push(name.to_string());
                state.add_import(name.to_string(), path.clone());
                path.pop();
            }
        }
        state
    }

    pub fn expected_var_is_ty(&mut self, id: u32, ty: Ty<'db>, span: Span) {
        if let Ty::TypeVar { id: second } = ty {
            self.type_state.merge(id, second);
            return;
        }
        let var = self.type_state.get_type_var_mut(id);
        if let Some(resolved) = var.resolved.clone() {
            resolved.expect_is_instance_of(&ty, self, false, span);
            return;
        }
        var.resolved = Some(ty);
    }

    pub fn expected_ty_is_var(&mut self, id: u32, ty: Ty<'db>, span: Span) {
        if let Ty::TypeVar { id: second } = ty {
            self.type_state.merge(id, second);
            return;
        }
        let var = self.type_state.get_type_var_mut(id);
        if let Some(resolved) = var.resolved.clone() {
            ty.expect_is_instance_of(&resolved, self, false, span);
            return;
        }
        var.resolved = Some(ty);
    }

    pub fn get_type_vars(&mut self) -> HashMap<u32, Ty<'db>> {
        let mut res = HashMap::new();
        let mut errors = vec![];
        for id in self.type_state.vars.keys().copied().collect::<Vec<_>>() {
            let ty = self.get_resolved_type_var(id);
            let data = self.type_state.get_type_var(id).clone();
            if let Ty::Unknown = ty {
                errors.push(UnboundTypeVar {
                    span: data.span,
                    file: data.file,
                });
            }
            // TODO: This should only error if the bound it not met. It shouldn't imply types
            // for bound in &data.bounds {
            //     ty.expect_is_instance_of(bound.super_.as_ref(), self, false, data.span);
            // }
            res.insert(id, ty);
        }
        for error in errors {
            self.error(CheckError::UnboundTypeVar(error));
        }
        res
    }

    pub fn add_import(&mut self, name: String, path: QualifiedName) {
        self.imports.insert(name, ModulePath::new(self.db, path));
    }

    pub fn enter_scope(&mut self) {
        self.variables.push(HashMap::new());
        self.generics.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        self.variables.pop();
        self.generics.pop();
    }

    pub fn simple_error(&mut self, message: &str, span: Span) {
        self.error(CheckError::Simple(Simple {
            message: message.to_string(),
            span,
            file: self.file_data,
        }));
    }

    pub fn error(&mut self, error: CheckError) {
        if self.should_error && self.path.first().unwrap() != "std" {
            Error { inner: error }
                .into_with_db(self.db)
                .accumulate(self.db);
        }
    }

    pub fn get_decl_with_error(
        &mut self,
        path: &[Spanned<String>],
    ) -> Result<Decl<'db>, Unresolved> {
        if let Some(import) = self.imports.get(&path[0].0).copied() {
            let module = self
                .project
                .decls(self.db)
                .get_path(self.db, import)
                .unwrap();
            module.try_get_path(self, &path[1..])
        } else {
            self.project.decls(self.db).try_get_path(self, path)
        }
    }

    pub fn import(&mut self, use_: &SpannedQualifiedName) -> Result<(), Unresolved> {
        if let Err(e) = self.get_decl_with_error(use_) {
            Err(e)
        } else {
            self.imports.insert(
                use_.last().unwrap().0.clone(),
                ModulePath::new(self.db, use_.iter().map(|(name, _)| name.clone()).collect()),
            );
            Ok(())
        }
    }

    pub fn insert_generic(&mut self, name: String, ty: Generic<'db>) {
        self.generics.last_mut().unwrap().insert(name, ty);
    }

    pub fn insert_variable(&mut self, name: String, ty: Ty<'db>, kind: TokenKind, span: Span) {
        let var = VarDecl {
            name: name.clone(),
            ty,
            kind,
            span,
        };
        self.variables.last_mut().unwrap().insert(name, var);
    }

    pub fn get_generic(&self, name: &str) -> Option<&Generic<'db>> {
        for generics in self.generics.iter().rev() {
            if let Some(g) = generics.get(name) {
                return Some(g);
            }
        }
        None
    }

    pub fn get_variable(&self, name: &str) -> Option<VarDecl<'db>> {
        for variables in self.variables.iter().rev() {
            if let Some(v) = variables.get(name) {
                return Some(v.clone());
            }
        }
        None
    }

    pub fn get_resolved_type_var(&self, id: u32) -> Ty<'db> {
        self.type_state
            .get_type_var(id)
            .resolved
            .clone()
            .unwrap_or(Ty::Unknown)
    }

    pub fn try_get_resolved_type_var(&self, id: u32) -> Option<Ty> {
        self.type_state.get_type_var(id).resolved.clone()
    }

    pub fn local_id(&self, name: String) -> ModulePath<'db> {
        let mut path = self.path.clone();
        path.push(name);
        ModulePath::new(self.db, path)
    }

    pub fn get_variables(&self) -> HashMap<String, VarDecl<'db>> {
        let mut vars = HashMap::new();
        for scope in &self.variables {
            for (name, var) in scope {
                vars.insert(name.clone(), var.clone());
            }
        }
        vars
    }

    pub fn get_generics(&self) -> HashMap<String, Generic<'db>> {
        let mut vars = HashMap::new();
        for scope in &self.generics {
            for (name, var) in scope {
                vars.insert(name.clone(), var.clone());
            }
        }
        vars
    }

    pub fn get_imports<'st>(&'st self) -> &'st HashMap<String, ModulePath<'db>> {
        &self.imports
    }

    pub fn get_decl(&self, name: ModulePath<'db>) -> Decl<'db> {
        self.try_get_decl_path(name).expect("Decl not found")
    }

    pub fn try_get_decl_path(&self, name: ModulePath<'db>) -> Option<Decl<'db>> {
        self.project.get_decl(self.db, name)
    }
}
