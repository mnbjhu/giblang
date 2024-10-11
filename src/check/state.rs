use std::{collections::HashMap, vec};

use salsa::Accumulator;

use crate::{
    check::err::{simple::Simple, CheckError},
    db::{
        input::{Db, SourceFile},
        modules::ModulePath,
    },
    parser::{expr::qualified_name::SpannedQualifiedName, parse_file},
    project::{name::QualifiedName, Project},
    ty::{Generic, Ty},
    util::Span,
};

use super::{
    err::{unresolved_type_var::UnboundTypeVar, Error},
    type_state::{MaybeTypeVar, TypeState, TypeVarUsage},
};

pub struct CheckState<'ty, 'db: 'ty> {
    pub db: &'db dyn Db,
    imports: HashMap<String, QualifiedName>,
    generics: Vec<HashMap<String, Generic<'db>>>,
    variables: Vec<HashMap<String, Ty<'db>>>,
    pub file_data: SourceFile,
    pub project: Project<'db>,
    pub type_state: TypeState<'ty, 'db>,
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
        };
        let mut path = file_data.module_path(db).name(db).clone();
        for top in parse_file(db, file_data).tops(db) {
            path.push(top.name(db));
            state.add_import(top.name(db), path.clone());
            path.pop();
        }
        state
    }

    pub fn add_import(&mut self, name: String, path: QualifiedName) {
        self.imports.insert(name, path);
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
        Error { inner: error }.accumulate(self.db);
    }

    pub fn get_decl_with_error(&mut self, path: &SpannedQualifiedName) -> Option<ModulePath<'db>> {
        if self
            .project
            .decls(self.db)
            .get_path_with_error(self.db, path.clone(), self.file_data)
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
                use_.iter().map(|(name, _)| name.clone()).collect(),
            );
        }
    }

    pub fn insert_generic(&mut self, name: String, ty: Generic<'db>) {
        self.generics.last_mut().unwrap().insert(name, ty);
    }

    pub fn insert_variable(&mut self, name: String, ty: Ty<'db>) {
        self.variables.last_mut().unwrap().insert(name, ty);
    }

    pub fn get_generic(&self, name: &str) -> Option<&Generic<'db>> {
        for generics in self.generics.iter().rev() {
            if let Some(g) = generics.get(name) {
                return Some(g);
            }
        }
        None
    }

    pub fn get_variable(&self, name: &str) -> Option<&Ty<'db>> {
        for variables in self.variables.iter().rev() {
            if let Some(v) = variables.get(name) {
                return Some(v);
            }
        }
        None
    }

    pub fn resolve_type_vars(&mut self) {
        let mut errs = vec![];
        let deferred = self
            .type_state
            .vars
            .values_mut()
            .filter_map(|var| {
                if let MaybeTypeVar::Data(data) = var {
                    data.resolve();
                    if data.resolved.is_none() || matches!(data.resolved, Some(Ty::Unknown)) {
                        errs.push(CheckError::UnboundTypeVar(UnboundTypeVar {
                            file: self.file_data,
                            span: data.span,
                            name: data
                                .bounds
                                .first()
                                .map_or("_".to_string(), |g| g.name.0.to_string()),
                        }));
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
                TypeVarUsage::VarIsTy(ty) => var
                    .unwrap()
                    .expect_is_instance_of(self.db, &ty.0, self, false, ty.1),
                TypeVarUsage::TyIsVar(ty) => {
                    ty.0.expect_is_instance_of(self.db, &var.unwrap(), self, false, ty.1)
                }
                _ => todo!("Check if needed"),
            };
        }
        for err in errs {
            self.error(err);
        }
    }

    pub fn get_resolved_type_var(&self, id: u32) -> Ty {
        self.type_state
            .get_type_var(id)
            .resolved
            .clone()
            .expect("Type var should be resolved")
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
