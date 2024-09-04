use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    io::Write,
    vec,
};

use ariadne::Source;
use glob::glob;

use crate::{
    check::{
        err::{unresolved::Unresolved, CheckError, ResolveError},
        state::CheckState,
    },
    parser::parse_file,
    project::{file_data::FileData, module::Node, util::path_from_filename},
    resolve::resolve_file,
    ty::{prim::PrimTy, Generic, Ty},
    util::{Span, Spanned},
};

use self::decl::Decl;

pub mod decl;
pub mod file_data;
pub mod inst;
mod module;
pub mod name;
pub mod util;

pub struct Project {
    pub root: Node,
    files: Vec<FileData>,
    parents: Vec<u32>,
    decls: HashMap<u32, Decl>,
    impls: HashMap<u32, ImplData>,
    impl_map: HashMap<u32, Vec<u32>>,
    counter: u32,
    pub valid: bool,
}

pub struct ImplData {
    pub id: u32,
    pub generics: Vec<Generic>,
    pub from: Ty,
    pub to: Ty,
    pub functions: Vec<u32>,
}

#[cfg(test)]
#[must_use]
pub fn check_test_state(project: &Project) -> CheckState {
    CheckState::from_file(project.get_file(0).unwrap(), project)
}

impl Project {
    #[allow(clippy::missing_panics_doc)]
    pub fn insert_file(&mut self, file_path: String, text: String) {
        let (ast, valid) = parse_file(
            &text,
            &file_path,
            &Source::from(text.clone()),
            &mut self.counter,
        );
        self.valid &= valid;
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

    #[must_use]
    pub fn get_file(&self, for_id: u32) -> Option<&FileData> {
        self.files.iter().find(|f| f.end >= for_id)
    }

    #[must_use]
    pub fn get_parent(&self, for_id: u32) -> Option<u32> {
        self.parents.iter().find(|&&id| id > for_id).copied()
    }

    pub fn insert_decl(&mut self, id: u32, decl: Decl) {
        self.decls.insert(id, decl);
    }

    #[cfg(test)]
    #[must_use]
    pub fn get_path(&self, path: &[&str]) -> Option<u32> {
        self.root.get_path(path)
    }

    /// # Errors
    ///
    /// This function will return an error if no path is found.
    pub fn get_path_with_error(
        &self,
        path: &[Spanned<String>],
        file: u32,
    ) -> Result<u32, Unresolved> {
        self.root.get_with_error(path, file)
    }

    #[must_use]
    pub fn get_path_without_error(&self, path: &[Spanned<String>]) -> Option<u32> {
        self.root.get_without_error(path)
    }

    #[must_use]
    pub fn init_pwd() -> Project {
        let mut project = Project::new();
        for file in glob("**/*.gib").unwrap() {
            let file = file.unwrap();
            let text = std::fs::read_to_string(&file).unwrap();
            project.insert_file(file.to_str().unwrap().to_string(), text);
        }
        project
    }

    #[must_use]
    pub fn get_decl(&self, id: u32) -> &Decl {
        self.decls
            .get(&id)
            .unwrap_or_else(|| panic!("Failed to resolve decl with id {id}"))
    }

    #[must_use]
    pub fn get_qualified_name(&self, id: u32) -> String {
        let file = self.get_file(id).unwrap();
        let mut name = file.get_path();
        let decl = self.get_decl(id);
        if matches!(decl, Decl::Member { .. }) {
            let parent = self
                .get_parent(id)
                .expect("Member decls should have a parent");
            let parent_decl = self.get_decl(parent);
            name.push(parent_decl.name());
        } else if matches!(decl, Decl::Function { .. }) {
            if let Some(parent) = self.get_parent(id) {
                let parent_decl = self.get_decl(parent);
                name.push(parent_decl.name());
            }
        };
        name.push(decl.name());
        name.join("::")
    }

    #[must_use]
    pub fn get_impls(&self, for_decl: u32) -> Vec<&ImplData> {
        let impl_ids = self.impl_map.get(&for_decl).cloned().unwrap_or_default();
        let mut impls = vec![];
        for id in &impl_ids {
            impls.push(self.impls.get(id).expect("Think these should be valid"));
        }
        impls
    }

    pub fn resolve(&mut self) -> Vec<ResolveError> {
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

    #[must_use]
    pub fn check(&self) -> Vec<CheckError> {
        let mut errors = vec![];
        for file in &self.files {
            let mut state = CheckState::from_file(file, self);
            for item in &file.ast {
                item.0.check(self, &mut state);
            }
            errors.extend(state.errors);
        }
        errors
    }

    pub fn build(&self) {
        for file in &self.files {
            let mut state = CheckState::from_file(file, self);
            let out = "main.go";
            let mut path = file.get_path();
            path.pop();
            let path = path.join("/");
            fs::create_dir_all(path).expect("Failed to create directory");
            let new = OpenOptions::new().create_new(true).append(true).open(out);
            let mut out = if let Ok(mut new) = new {
                new.write_all(
                    r#"package main
import "fmt"
"#
                    .to_string()
                    .as_bytes(),
                )
                .expect("Failed to write to file");
                new
            } else {
                OpenOptions::new().append(true).open(out).unwrap()
            };

            for item in &file.ast {
                let mut text = item.0.build(&mut state);
                text.push('\n');
                out.write_all(text.as_bytes())
                    .expect("Failed to write to file");
            }
        }
    }

    pub fn check_with_errors(&mut self) {
        let mut valid = true;
        for file in &self.files {
            let mut state = CheckState::from_file(file, self);
            for item in &file.ast {
                item.0.check(self, &mut state);
            }
            state.resolve_type_vars();
            for err in &state.errors {
                state.print_error(err);
                valid = false;
            }
        }
        self.valid &= valid;
    }

    #[must_use]
    pub fn new() -> Project {
        let mut decls = HashMap::new();
        decls.insert(1, Decl::Prim(PrimTy::String));
        decls.insert(2, Decl::Prim(PrimTy::Int));
        decls.insert(3, Decl::Prim(PrimTy::Bool));
        decls.insert(4, Decl::Prim(PrimTy::Float));
        decls.insert(5, Decl::Prim(PrimTy::Char));
        decls.insert(
            6,
            Decl::Function {
                name: ("print".to_string(), Span::splat(0)),
                generics: vec![],
                receiver: None,
                args: vec![("text".to_string(), Ty::string())],
                ret: Ty::unit(),
            },
        );

        let mut root = Node::module("root".to_string());
        root.insert(&[], 1, "String");
        root.insert(&[], 2, "Int");
        root.insert(&[], 3, "Bool");
        root.insert(&[], 4, "Float");
        root.insert(&[], 5, "Char");
        root.insert(&[], 6, "print");
        Project {
            root,
            files: vec![],
            parents: vec![],
            decls,
            impls: HashMap::new(),
            impl_map: HashMap::new(),
            counter: 6,
            valid: true,
        }
    }

    #[must_use]
    pub fn get_counter(&self) -> u32 {
        self.counter
    }

    #[must_use]
    pub fn get_impl(&self, id: &u32) -> &ImplData {
        self.impls.get(id).expect("Invalid impl id")
    }
}

#[derive(Debug, Clone)]
pub struct TypeVar {
    pub id: u32,
    pub generic: Generic,
    pub ty: Option<Ty>,
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
        #[must_use]
        pub fn from(text: &str) -> Project {
            let mut project = Project::new();
            project.insert_file("main.gib".to_string(), text.to_string());
            project
        }

        #[must_use]
        pub fn check_test() -> Project {
            let mut project = Project::from(
                r"struct Foo
struct Bar[T]
struct Baz[T, U]
enum Option[out T] {
   Some(T),
   None
}
enum Result[out R, out E] {
   Ok(R),
   Err(E),
}
fn add(a: Int, b: Int): Int { }
fn Int.factorial(): Int { }
fn ident[T](t: T): T { }
",
            );
            project.resolve();
            project
        }
    }
}
