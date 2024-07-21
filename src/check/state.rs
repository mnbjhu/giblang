use std::collections::HashMap;

use ariadne::Source;

use crate::{
    fs::{name::QualifiedName, util::path_from_filename},
    parser::{expr::qualified_name::SpannedQualifiedName, top::Top, File},
    project::{file_data::FileData, Project},
    ty::{Generic, Ty},
    util::{Span, Spanned},
};

pub struct CheckState<'file> {
    imports: HashMap<String, QualifiedName>,
    decls: HashMap<String, u32>,
    generics: Vec<HashMap<String, Generic>>,
    variables: Vec<HashMap<String, Ty>>,
    file_data: &'file FileData,
    project: &'file Project,
}

impl<'file> CheckState<'file> {
    pub fn from_file(file_data: &'file FileData, project: &'file Project) -> CheckState<'file> {
        let source = Source::from(file_data.text.clone());
        let mut state = CheckState {
            imports: HashMap::new(),
            decls: HashMap::new(),
            generics: vec![],
            variables: vec![],
            file_data,
            project,
        };
        for (top, _) in &file_data.ast {
            if let Some(name) = top.get_name() {
                let id = top.get_id().unwrap();
                let mut path = path_from_filename(&file_data.name);
                state.insert_decl(name.to_string(), id)
            }
        }
        state
    }

    pub fn import_all(&mut self, file: &File, project: &Project) {
        for top in file {
            if let Top::Use(path) = &top.0 {
                self.import(path);
            }
        }
    }

    pub fn enter_scope(&mut self) {
        self.variables.push(HashMap::new());
        self.generics.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        self.variables.pop();
        self.generics.pop();
    }

    pub fn error(&self, message: &str, span: Span) {
        self.file_data.error(message, span);
    }

    pub fn get_decl_with_error(&self, path: &SpannedQualifiedName) -> Option<u32> {
        let name = path[0].0;
        if path.len() == 1 {
            if let Some(decl) = self.decls.get(&name) {
                return Some(*decl);
            }
        }
        if let Some(import) = self.imports.get(&name) {
            let module = self
                .project
                .root
                .get_module(import, self.file_data)
                .expect("There should only be valid paths at this point??");
            module.get_with_error(&path[1..], self.file_data)
        } else {
            self.project.get_path_with_error(path, self.file_data)
        }
    }

    pub fn import(&mut self, use_: &SpannedQualifiedName) {
        if let Some(decl) = self.get_decl_with_error(use_) {
            if decl == 0 {
                self.imports.insert(
                    use_.last().unwrap().0.clone(),
                    use_.iter().map(|(name, _)| name.clone()).collect(),
                );
            } else {
                self.decls.insert(use_.last().unwrap().0.clone(), decl);
            }
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

    pub fn insert_decl(&mut self, name: String, id: u32) {
        self.decls.insert(name, id);
    }

    pub fn get_expr(&self, path: &[Spanned<String>]) -> Ty {
        todo!()
    }
}
