use std::collections::HashMap;

use salsa::Accumulator;

use crate::{
    check::err::{CheckError, Error},
    db::{
        input::{Db, SourceFile},
        modules::ModulePath,
    },
    parser::{common::variance::Variance, expr::qualified_name::SpannedQualifiedName, parse_file},
    project::{name::QualifiedName, Project},
    ty::{Generic, Ty},
    util::{Span, Spanned},
};

pub struct ResolveState<'db> {
    imports: HashMap<String, ModulePath<'db>>,
    generics: Vec<HashMap<String, Generic<'db>>>,
    file_data: SourceFile,
    pub project: Project<'db>,
}

impl<'db> ResolveState<'db> {
    pub fn add_self_ty(&mut self, super_: Ty<'db>, span: Span) {
        self.insert_generic(
            "Self".to_string(),
            Generic {
                name: ("Self".to_string(), span),
                variance: Variance::Invariant,
                super_: Box::new(super_),
            },
        );
    }
    pub fn from_file(
        db: &'db dyn Db,
        file_data: SourceFile,
        project: Project<'db>,
    ) -> ResolveState<'db> {
        let mut state = ResolveState {
            imports: HashMap::new(),
            generics: vec![],
            file_data,
            project,
        };
        let mut path = file_data.module_path(db);
        for top in parse_file(db, file_data).tops(db) {
            if let Some(name) = top.get_name() {
                path.push(name.to_string());
                state.add_import(name.to_string(), path.clone());
                path.pop();
            }
        }
        state
    }

    pub fn add_import(&mut self, name: String, path: ModulePath<'db>) {
        self.imports.insert(name, path);
    }

    pub fn enter_scope(&mut self) {
        self.generics.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        self.generics.pop();
    }

    pub fn error(&mut self, db: &'db dyn Db, error: Error) {
        error.accumulate(db)
    }

    pub fn get_decl_with_error(&mut self, path: &SpannedQualifiedName) -> Option<ModulePath> {
        let name = path[0].0.clone();
        let res = if let Some(import) = self.imports.get(&name) {
            let module = self.project.root.get_module(import)?;
            module.get_with_error(&path[1..], self.file_data.end)
        } else {
            self.project.get_path_with_error(path, self.file_data.end)
        };
        match res {
            Ok(decl) => Some(decl),
            Err(e) => {
                self.error(ResolveError::Unresolved(e));
                None
            }
        }
    }

    pub fn get_decl_without_error(&self, path: &[Spanned<String>]) -> Option<ModulePath> {
        let name = path.first()?;
        if let Some(import) = self.imports.get(&name.0) {
            let module = self
                .project
                .root
                .get_module(import)
                .expect("There should only be valid paths at this point??");
            module.get_without_error(&path[1..])
        } else {
            self.project.get_path_without_error(path)
        }
    }

    pub fn import(&mut self, db: &'db dyn Db, use_: &SpannedQualifiedName) {
        if self.get_decl_with_error(use_).is_some() {
            let path = use_.iter().map(|(name, _)| name.to_string()).collect();
            self.imports
                .insert(use_.last().unwrap().0.clone(), ModulePath::new(db, path));
        }
    }

    pub fn insert_generic(&mut self, name: String, ty: Generic<'db>) {
        self.generics.last_mut().unwrap().insert(name, ty);
    }

    pub fn get_generic(&self, name: &str) -> Option<&Generic<'db>> {
        for generics in self.generics.iter().rev() {
            if let Some(g) = generics.get(name) {
                return Some(g);
            }
        }
        None
    }

    pub fn get_file(&self) -> u32 {
        self.file_data.end
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
