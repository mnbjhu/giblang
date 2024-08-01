use std::collections::HashMap;

use ariadne::Source;
use glob::glob;

use crate::{
    check::state::CheckState,
    parser::parse_file,
    project::{file_data::FileData, module::ModuleNode, util::path_from_filename},
    resolve::{resolve_file, top::Decl},
    ty::{Generic, PrimTy, Ty},
    util::Spanned,
};

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
}

pub struct ImplData {
    pub generics: Vec<Generic>,
    pub from: Ty,
    pub to: Ty,
    pub functions: Vec<u32>,
}

impl Project {
    pub fn insert_file(&mut self, text: String, name: String, counter: &mut u32) {
        let ast = parse_file(&text, &name, &Source::from(text.clone()), counter);
        let mut path = path_from_filename(&name);
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
            end: *counter,
            ast,
            text,
            name,
        };
        self.files.push(file_data);
    }

    pub fn get_file(&self, for_id: u32) -> Option<&FileData> {
        self.files.iter().find(|f| f.end > for_id)
    }

    pub fn get_parent(&self, for_id: u32) -> Option<u32> {
        self.parents.iter().find(|&&id| id > for_id).copied()
    }

    pub fn insert_decl(&mut self, id: u32, decl: Decl) {
        self.decls.insert(id, decl);
    }

    pub fn get_path_with_error(&self, path: &[Spanned<String>], file: &FileData) -> Option<u32> {
        self.root.get_with_error(path, file)
    }
    pub fn get_path_without_error(&self, path: &[Spanned<String>]) -> Option<u32> {
        self.root.get_without_error(path)
    }

    pub fn init_pwd() -> Project {
        let mut counter = 6;
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
        let mut project = Project {
            root,
            files: vec![],
            parents: vec![],
            decls,
            impls: HashMap::new(),
            impl_map: HashMap::new(),
        };
        for file in glob("**/*.gib").unwrap() {
            let file = file.unwrap();
            let text = std::fs::read_to_string(&file).unwrap();
            project.insert_file(text, file.to_str().unwrap().to_string(), &mut counter);
        }
        project
    }

    pub fn get_decl(&self, id: u32) -> &Decl {
        self.decls
            .get(&id)
            .expect("Should only be called by types, type should be unresolved at this point")
    }

    pub fn get_impls(&self, for_decl: u32) -> Vec<&ImplData> {
        let impl_ids = self.impl_map.get(&for_decl).cloned().unwrap_or_default();
        let mut impls = vec![];
        for id in &impl_ids {
            impls.push(self.impls.get(id).expect("Think these should be valid"))
        }
        impls
    }

    pub fn resolve(&mut self) {
        let mut decls = HashMap::new();
        let mut impls = HashMap::new();
        let mut impl_map = HashMap::new();
        self.files
            .iter()
            .for_each(|file| resolve_file(file, &mut decls, &mut impls, &mut impl_map, self));
        self.decls.extend(decls);
        self.impls = impls;
        self.impl_map = impl_map;
    }

    pub fn check(&self) {
        for file in &self.files {
            let mut state = CheckState::from_file(file, self);
            for item in &file.ast {
                item.0.check(self, &mut state)
            }
        }
    }
}
