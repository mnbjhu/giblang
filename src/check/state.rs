use std::collections::HashMap;

use crate::{
    check::err::{simple::Simple, CheckError},
    parser::expr::qualified_name::SpannedQualifiedName,
    project::{file_data::FileData, name::QualifiedName, Project},
    ty::{Generic, Ty},
    util::{Span, Spanned},
};

pub struct CheckState<'file> {
    imports: HashMap<String, QualifiedName>,
    generics: Vec<HashMap<String, Generic>>,
    variables: Vec<HashMap<String, Ty>>,
    pub file_data: &'file FileData,
    pub project: &'file Project,
    pub errors: Vec<CheckError>,
}

impl<'file> CheckState<'file> {
    pub fn from_file(file_data: &'file FileData, project: &'file Project) -> CheckState<'file> {
        let mut state = CheckState {
            imports: HashMap::new(),
            generics: vec![],
            variables: vec![],
            file_data,
            project,
            errors: vec![],
        };
        let mut path = file_data.get_path();
        for (top, _) in &file_data.ast {
            if let Some(name) = top.get_name() {
                path.push(name.to_string());
                state.add_import(name.to_string(), path.clone());
                path.pop();
            }
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
        self.errors.push(CheckError::Simple(Simple {
            message: message.to_string(),
            span,
            file: self.file_data.end,
        }));
    }

    pub fn error(&mut self, error: CheckError) {
        self.errors.push(error);
    }

    pub fn get_decl_with_error(&mut self, path: &SpannedQualifiedName) -> Option<u32> {
        let name = path[0].0.clone();
        if let Some(import) = self.imports.get(&name) {
            let module = self.project.root.get_module(import)?;
            module.get_with_error(&path[1..], self)
        } else {
            self.project.get_path_with_error(path, self)
        }
    }

    pub fn get_decl_without_error(&self, path: &[Spanned<String>]) -> Option<u32> {
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

    pub fn import(&mut self, use_: &SpannedQualifiedName) {
        if self.get_decl_with_error(use_).is_some() {
            self.imports.insert(
                use_.last().unwrap().0.clone(),
                use_.iter().map(|(name, _)| name.clone()).collect(),
            );
        }
    }

    pub fn insert_generic(&mut self, name: String, ty: Generic) {
        self.generics.last_mut().unwrap().insert(name, ty);
    }

    pub fn insert_variable(&mut self, name: String, ty: Ty) {
        self.variables.last_mut().unwrap().insert(name, ty);
    }

    pub fn get_generic(&self, name: &str) -> Option<&Generic> {
        for generics in self.generics.iter().rev() {
            if let Some(g) = generics.get(name) {
                return Some(g);
            }
        }
        None
    }

    pub fn get_variable(&self, name: &str) -> Option<&Ty> {
        for variables in self.variables.iter().rev() {
            if let Some(v) = variables.get(name) {
                return Some(v);
            }
        }
        None
    }

    #[allow(unused)]
    pub fn get_expr(&self, _: &[Spanned<String>]) -> Ty {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        check::{state::CheckState, ty::tests::parse_ty},
        project::Project,
        ty::{Generic, Ty},
    };

    fn test_project() -> Project {
        let mut project = Project::from(
            r#"struct Foo
            struct Bar
            struct Baz[T]
            trait Magic {
                fn magic(): Self
            }
            trait Epic {
                fn epic(): Self
            }
            trait Strange [T] {
                fn strange(): T
            }

            impl Magic for Foo

            impl Magic for Bar
            impl Epic for Bar

            impl Strange[T] for Baz[T]"#,
        );
        project.resolve();
        project
    }

    fn test_state<'a>(project: &'a Project) -> CheckState<'a> {
        let file_data = project.get_file(0).unwrap();
        CheckState::from_file(file_data, project)
    }

    #[test]
    fn variables() {
        let project = test_project();
        let mut state = test_state(&project);
        state.enter_scope();
        state.insert_variable("foo".to_string(), parse_ty(&project, "Foo"));
        assert_eq!(
            *state.get_variable("foo").unwrap(),
            parse_ty(&project, "Foo")
        );
        state.exit_scope();
        assert!(state.get_variable("foo").is_none());
    }

    #[test]
    fn generics() {
        let project = test_project();
        let mut state = test_state(&project);
        state.enter_scope();
        state.insert_generic(
            "T".to_string(),
            Generic {
                name: "T".to_string(),
                ..Default::default()
            },
        );
        assert_eq!(
            *state.get_generic("T").unwrap(),
            Generic {
                name: "T".to_string(),
                ..Default::default()
            }
        );
        state.exit_scope();
        assert!(state.get_generic("T").is_none());
    }
}
