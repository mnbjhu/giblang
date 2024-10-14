use std::{collections::HashMap, vec};

use salsa::Accumulator;

use crate::{
    check::err::{simple::Simple, CheckError},
    db::{
        input::{Db, SourceFile},
        modules::{Module, ModuleData, ModulePath},
    },
    parser::{expr::qualified_name::SpannedQualifiedName, parse_file},
    project::{name::QualifiedName, Project},
    ty::{Generic, Ty},
    util::{Span, Spanned},
};

use super::{
    err::{unresolved_type_var::UnboundTypeVar, Error, IntoWithDb},
    type_state::{MaybeTypeVar, TypeState, TypeVarUsage},
};

#[derive(Debug, Clone)]
pub struct VarDecl<'db> {
    pub ty: Ty<'db>,
    pub is_param: bool,
}

pub struct CheckState<'ty, 'db: 'ty> {
    pub db: &'db dyn Db,
    imports: HashMap<String, ModulePath<'db>>,
    generics: Vec<HashMap<String, Generic<'db>>>,
    variables: Vec<HashMap<String, VarDecl<'db>>>,
    pub file_data: SourceFile,
    pub project: Project<'db>,
    pub type_state: TypeState<'ty, 'db>,
    pub path: Vec<String>,
    pub should_error: bool,
}

impl<'ty, 'db: 'ty> CheckState<'ty, 'db> {
    pub fn from_file(
        db: &'db dyn Db,
        file_data: SourceFile,
        project: Project<'db>,
    ) -> CheckState<'db, 'ty> {
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

    pub fn get_type_vars(&self) -> HashMap<u32, Ty<'db>> {
        let mut res = HashMap::new();
        for id in self.type_state.vars.keys() {
            res.insert(*id, self.get_resolved_type_var(*id));
        }
        res
    }

    pub fn with_type_vars(&mut self, types: HashMap<u32, Ty<'db>>) {
        for (id, ty) in types {
            self.type_state.new_type_var(Span::splat(0));
            let var = self.type_state.get_type_var_mut(id);
            var.resolved = Some(ty);
        }
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
        if self.should_error {
            Error { inner: error }
                .into_with_db(self.db)
                .accumulate(self.db);
        }
    }

    pub fn get_module_with_error(&mut self, path: &[Spanned<String>]) -> Option<Module<'db>> {
        if let Some(import) = self.imports.get(&path[0].0).copied() {
            let module = self
                .project
                .decls(self.db)
                .get_path(self.db, import)
                .unwrap();
            module.get_path_with_state(self, &path[1..], self.file_data, self.should_error)
        } else {
            self.project.decls(self.db).get_path_with_state(
                self,
                path,
                self.file_data,
                self.should_error,
            )
        }
    }

    pub fn get_decl_with_error(&mut self, path: &[Spanned<String>]) -> Option<ModulePath<'db>> {
        if let Some(import) = self.imports.get(&path[0].0).copied() {
            let module = self
                .project
                .decls(self.db)
                .get_path(self.db, import)
                .unwrap();
            if module
                .get_path_with_state(self, &path[1..], self.file_data, self.should_error)
                .is_some()
            {
                let mut new = import.name(self.db).clone();
                new.extend(path[1..].iter().map(|(n, _)| n.to_string()));
                Some(ModulePath::new(self.db, new))
            } else {
                None
            }
        } else if let Some(found) = self.project.decls(self.db).get_path_with_state(
            self,
            path,
            self.file_data,
            self.should_error,
        ) {
            if let ModuleData::Export(_) = found.content(self.db) {
                Some(ModulePath::new(
                    self.db,
                    path.iter().map(|(n, _)| n.to_string()).collect(),
                ))
            } else {
                self.error(CheckError::Simple(Simple {
                    message: "Expected export".to_string(),
                    span: path.last().unwrap().1,
                    file: self.file_data,
                }));
                None
            }
        } else {
            None
        }
    }

    pub fn get_decl_without_error(&self, path: &SpannedQualifiedName) -> Option<ModulePath<'db>> {
        if self
            .project
            .decls(self.db)
            .get_path_without_error(self.db, path.clone())
            .is_some()
        {
            Some(ModulePath::new(
                self.db,
                path.iter().map(|(n, _)| n.to_string()).collect(),
            ))
        } else {
            None
        }
    }

    pub fn import(&mut self, use_: &SpannedQualifiedName) {
        if self.get_decl_with_error(use_).is_some() {
            self.imports.insert(
                use_.last().unwrap().0.clone(),
                ModulePath::new(self.db, use_.iter().map(|(name, _)| name.clone()).collect()),
            );
        }
    }

    pub fn insert_generic(&mut self, name: String, ty: Generic<'db>) {
        self.generics.last_mut().unwrap().insert(name, ty);
    }

    pub fn insert_variable(&mut self, name: String, ty: Ty<'db>, param: bool) {
        let var = VarDecl {
            ty,
            is_param: param,
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

    pub fn resolve_type_vars(&mut self) {
        let deferred = self
            .type_state
            .vars
            .values_mut()
            .filter_map(|var| {
                if let MaybeTypeVar::Data(data) = var {
                    data.resolve();
                    if data.resolved.is_none() || matches!(data.resolved, Some(Ty::Unknown)) {
                        UnboundTypeVar {
                            file: self.file_data,
                            span: data.span,
                            name: data
                                .bounds
                                .first()
                                .map_or("_".to_string(), |g| g.name.0.to_string()),
                        }
                        .into_with_db(self.db)
                        .accumulate(self.db);
                        None
                    } else {
                        Some(data)
                    }
                } else {
                    None
                }
            })
            .flat_map(|data| {
                data.usages
                    .iter()
                    .map(|u| (data.resolved.clone(), u.clone()))
            })
            .collect::<Vec<_>>();

        for (var, usage) in deferred {
            match usage {
                TypeVarUsage::VarIsTy(ty) => {
                    var.unwrap().expect_is_instance_of(&ty.0, self, false, ty.1)
                }
                TypeVarUsage::TyIsVar(ty) => {
                    ty.0.expect_is_instance_of(&var.unwrap(), self, false, ty.1)
                }
                _ => todo!("Check if needed"),
            };
        }
    }

    pub fn get_resolved_type_var(&self, id: u32) -> Ty<'db> {
        self.type_state
            .get_type_var(id)
            .resolved
            .clone()
            .expect("Type var should be resolved")
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
}

// #[cfg(test)]
// mod tests {
//     use crate::{
//         check::{state::CheckState, ty::tests::parse_ty},
//         project::Project,
//         ty::Generic,
//         util::Span,
//     };
//
//     fn test_project() -> Project {
//         let mut project = Project::from(
//             r"struct Foo
//             struct Bar
//             struct Baz[T]
//             trait Magic {
//                 fn magic(): Self
//             }
//             trait Epic {
//                 fn epic(): Self
//             }
//             trait Strange [T] {
//                 fn strange(): T
//             }
//
//             impl Magic for Foo
//
//             impl Magic for Bar
//             impl Epic for Bar
//
//             impl Strange[T] for Baz[T]",
//         );
//         project.resolve();
//         project
//     }
//
//     fn test_state(project: &Project) -> CheckState {
//         let file_data = project.get_file(0).unwrap();
//         CheckState::from_file(file_data, project)
//     }
//
//     #[test]
//     fn variables() {
//         let project = test_project();
//         let mut state = test_state(&project);
//         state.enter_scope();
//         state.insert_variable("foo".to_string(), parse_ty(&project, "Foo"));
//         assert_eq!(
//             *state.get_variable("foo").unwrap(),
//             parse_ty(&project, "Foo")
//         );
//         state.exit_scope();
//         assert!(state.get_variable("foo").is_none());
//     }
//
//     #[test]
//     fn generics() {
//         let project = test_project();
//         let mut state = test_state(&project);
//         state.enter_scope();
//         state.insert_generic(
//             "T".to_string(),
//             Generic::new(("T".to_string(), Span::splat(0))),
//         );
//         assert_eq!(
//             *state.get_generic("T").unwrap(),
//             Generic::new(("T".to_string(), Span::splat(0))),
//         );
//         state.exit_scope();
//         assert!(state.get_generic("T").is_none());
//     }
// }
