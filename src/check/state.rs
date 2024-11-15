use std::{collections::HashMap, vec};

use salsa::{Accumulator, Update};

use crate::{
    check::err::{simple::Simple, CheckError},
    db::{
        decl::{Decl, Project},
        input::{Db, SourceFile},
        path::ModulePath,
    },
    ir::common::pattern::SpannedQualifiedNameIR,
    item::definitions::ident::IdentDef,
    parser::{common::variance::Variance, expr::qualified_name::SpannedQualifiedName, parse_file},
    ty::{Generic, Ty},
    util::{Span, Spanned},
};

use super::{
    err::{unresolved::Unresolved, unresolved_type_var::UnboundTypeVar, Error, IntoWithDb},
    type_state::TypeState,
    TokenKind,
};

#[derive(Debug, PartialEq, Clone, Update, Eq)]
pub struct VarDecl<'db> {
    pub name: String,
    pub ty: Ty<'db>,
    pub kind: TokenKind,
    pub span: Span,
}

pub struct CheckState<'db> {
    pub db: &'db dyn Db,
    pub imports: HashMap<String, Decl<'db>>,
    generics: Vec<HashMap<String, Generic<'db>>>,
    variables: Vec<HashMap<String, VarDecl<'db>>>,
    pub file_data: SourceFile,
    pub project: Project<'db>,
    pub type_state: TypeState<'db>,
    pub path: Vec<String>,
    pub should_error: bool,
    pub file_decl: Decl<'db>,
}

impl<'ty, 'db: 'ty> CheckState<'db> {
    pub fn from_file(
        db: &'db dyn Db,
        file_data: SourceFile,
        project: Project<'db>,
    ) -> CheckState<'db> {
        let path = file_data.module_path(db);
        let decl = project.get_decl(db, path).unwrap();
        let mut state = CheckState {
            db,
            imports: HashMap::new(),
            generics: vec![],
            variables: vec![],
            file_data,
            project,
            type_state: TypeState::default(),
            should_error: true,
            file_decl: decl,
            path: path.name(db).clone(),
        };
        let tops = parse_file(db, file_data).tops(db);
        for top in tops {
            if let Some(name) = top.0.get_name() {
                state.add_local_import(name);
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
            resolved.expect_is_instance_of(&ty, self, span);
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
            ty.expect_is_instance_of(&resolved, self, span);
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

    pub fn add_local_import(&mut self, name: &str) {
        self.imports
            .insert(name.into(), self.file_decl.get(self.db, name).unwrap());
    }

    pub fn enter_scope(&mut self) {
        self.variables.push(HashMap::new());
        self.generics.push(HashMap::new());
    }

    #[must_use]
    pub fn exit_scope(&mut self) -> (HashMap<String, VarDecl<'db>>, HashMap<String, Generic<'db>>) {
        let vars = self.variables.pop().unwrap();
        let generics = self.generics.pop().unwrap();
        (vars, generics)
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
            import.try_get_path(self, &path[1..])
        } else {
            self.project.decls(self.db).try_get_path(self, path)
        }
    }

    pub fn add_self_ty(&mut self, super_: &Ty<'db>, span: Span) {
        let generic = Generic {
            name: ("Self".to_string(), span),
            variance: Variance::Invariant,
            super_: Box::new(super_.clone()),
        };
        self.insert_generic("Self".to_string(), generic);
    }

    pub fn add_self_param(&mut self, ty: Ty<'db>, span: Span) {
        if let Ty::Generic(g) = ty {
            if g.name.0 == "Self" {
                self.insert_variable("self".to_string(), *g.super_, TokenKind::Param, span);
            }
        } else {
            self.insert_variable("self".to_string(), ty, TokenKind::Param, span);
        }
    }

    pub fn get_ident_ir(&mut self, path: &[Spanned<String>]) -> SpannedQualifiedNameIR<'db> {
        if let Some(import) = self.imports.get(&path[0].0).copied() {
            let mut found = import.get_path_ir(self, &path[1..]);
            found.insert(0, (IdentDef::Decl(import), path[0].1));
            found
        } else {
            self.project.decls(self.db).get_path_ir(self, path)
        }
    }

    pub fn get_ident_ir_with_error(
        &mut self,
        path: &[Spanned<String>],
    ) -> SpannedQualifiedNameIR<'db> {
        if let Some(import) = self.imports.get(&path[0].0).copied() {
            let mut found = import.get_path_ir_with_error(self, &path[1..]);
            found.insert(0, (IdentDef::Decl(import), path[0].1));
            found
        } else {
            self.project
                .decls(self.db)
                .get_path_ir_with_error(self, path)
        }
    }

    pub fn import(&mut self, use_: &SpannedQualifiedName) -> Result<(), Unresolved> {
        match self.get_decl_with_error(use_) {
            Err(e) => Err(e),
            Ok(decl) => {
                self.imports.insert(use_.last().unwrap().0.clone(), decl);
                Ok(())
            }
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

    pub fn get_imports<'st>(&'st self) -> &'st HashMap<String, Decl<'db>> {
        &self.imports
    }

    pub fn try_get_decl_path(&self, name: ModulePath<'db>) -> Option<Decl<'db>> {
        self.project.get_decl(self.db, name)
    }
}
