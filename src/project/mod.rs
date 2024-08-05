use std::collections::HashMap;

use ariadne::{Color, Source};
use glob::glob;

use crate::{
    check::state::{CheckError, CheckState},
    parser::parse_file,
    project::{file_data::FileData, module::ModuleNode, util::path_from_filename},
    resolve::resolve_file,
    ty::{prim::PrimTy, Generic, Ty},
    util::Spanned,
};

use self::decl::Decl;

pub mod decl;
pub mod file_data;
mod module;
pub mod name;
pub mod util;

pub struct Project {
    pub root: ModuleNode,
    files: Vec<FileData>,
    parents: Vec<u32>,
    decls: HashMap<u32, Decl>,
    impls: HashMap<u32, ImplData>,
    impl_map: HashMap<u32, Vec<u32>>,
    counter: u32,
}

pub struct ImplData {
    pub generics: Vec<Generic>,
    pub from: Ty,
    pub to: Ty,
    pub functions: Vec<u32>,
}

impl Project {
    pub fn insert_file(&mut self, file_path: String, text: String) {
        let ast = parse_file(
            &text,
            &file_path,
            &Source::from(text.clone()),
            &mut self.counter,
        );
        let mut path = path_from_filename(&file_path);
        for item in &ast {
            if let Some(name) = item.0.get_name() {
                let id = item.0.get_id().unwrap();
                if item.0.is_parent() {
                    self.parents.push(id);
                }
                self.root.insert(&path, id, name);
                for (child_name, id) in &item.0.children() {
                    path.push(name.to_string());
                    self.root.insert(&path, *id, child_name);
                    path.pop();
                }
            }
        }
        let file_data = FileData {
            end: self.counter,
            ast,
            text,
            name: file_path,
        };
        self.files.push(file_data);
    }

    pub fn get_file(&self, for_id: u32) -> Option<&FileData> {
        self.files.iter().find(|f| f.end >= for_id)
    }

    pub fn get_parent(&self, for_id: u32) -> Option<u32> {
        self.parents.iter().find(|&&id| id > for_id).copied()
    }

    // TODO: Delete if not needed
    #[allow(dead_code)]
    pub fn insert_decl(&mut self, id: u32, decl: Decl) {
        self.decls.insert(id, decl);
    }

    pub fn get_path(&self, path: &[&str]) -> Option<u32> {
        self.root.get_path(path)
    }

    pub fn get_path_with_error(&self, path: &[Spanned<String>], file: &FileData) -> Option<u32> {
        self.root.get_with_error(path, file)
    }

    pub fn get_path_without_error(&self, path: &[Spanned<String>]) -> Option<u32> {
        self.root.get_without_error(path)
    }

    pub fn init_pwd() -> Project {
        let mut project = Project::new();
        for file in glob("**/*.gib").unwrap() {
            let file = file.unwrap();
            let text = std::fs::read_to_string(&file).unwrap();
            project.insert_file(file.to_str().unwrap().to_string(), text);
        }
        project
    }

    pub fn get_decl(&self, id: u32) -> &Decl {
        self.decls
            .get(&id)
            .unwrap_or_else(|| panic!("Failed to resolve decl with id {}", id))
    }

    pub fn get_impls(&self, for_decl: u32) -> Vec<&ImplData> {
        let impl_ids = self.impl_map.get(&for_decl).cloned().unwrap_or_default();
        let mut impls = vec![];
        for id in &impl_ids {
            impls.push(self.impls.get(id).expect("Think these should be valid"))
        }
        impls
    }

    pub fn resolve(&mut self) -> Vec<CheckError> {
        let mut decls = HashMap::new();
        let mut impls = HashMap::new();
        let mut impl_map = HashMap::new();
        let mut errors = vec![];
        self.files.iter().for_each(|file| {
            let err = resolve_file(file, &mut decls, &mut impls, &mut impl_map, self);
            errors.extend(err);
        });
        self.decls.extend(decls);
        self.impls = impls;
        self.impl_map = impl_map;
        errors
    }

    pub fn check(&self) -> Vec<CheckError> {
        let mut errors = vec![];
        for file in &self.files {
            let mut state = CheckState::from_file(file, self);
            for item in &file.ast {
                item.0.check(self, &mut state)
            }
            errors.extend(state.errors);
        }
        errors
    }

    pub fn new() -> Project {
        let mut decls = HashMap::new();
        decls.insert(1, Decl::Prim(PrimTy::String));
        decls.insert(2, Decl::Prim(PrimTy::Int));
        decls.insert(3, Decl::Prim(PrimTy::Bool));
        decls.insert(4, Decl::Prim(PrimTy::Float));
        decls.insert(5, Decl::Prim(PrimTy::Char));

        let mut root = ModuleNode::module("root".to_string());
        root.insert(&[], 1, "String");
        root.insert(&[], 2, "Int");
        root.insert(&[], 3, "Bool");
        root.insert(&[], 4, "Float");
        root.insert(&[], 5, "Char");
        Project {
            root,
            files: vec![],
            parents: vec![],
            decls,
            impls: HashMap::new(),
            impl_map: HashMap::new(),
            counter: 6,
        }
    }

    pub fn print_error(&self, error: &CheckError) {
        let CheckError::Simple {
            message,
            span,
            file,
        } = error;

        let file_data = self
            .get_file(*file)
            .unwrap_or_else(|| panic!("No file found for id {}", file));
        let source = Source::from(file_data.text.clone());
        let name = &file_data.name;

        let err = Color::Red;

        let mut builder = ariadne::Report::build(ariadne::ReportKind::Error, name, span.start)
            .with_message(message.to_string())
            .with_code("error");

        builder = builder.with_label(
            ariadne::Label::new((name, span.into_range()))
                .with_message(message)
                .with_color(err),
        );

        let report = builder.finish();
        report.print((name, source)).unwrap();
    }
}

impl Default for Project {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Project;

    impl Project {
        pub fn from(text: &str) -> Project {
            let mut project = Project::new();
            project.insert_file("main.gib".to_string(), text.to_string());
            project
        }

        pub fn check_test() -> Project {
            let mut project = Project::from(
                r#"struct Foo
            struct Bar[T]
            struct Baz[T, U]"#,
            );
            project.resolve();
            project
        }

        #[allow(dead_code)]
        pub fn get_counter(&self) -> u32 {
            self.counter
        }
    }
}
