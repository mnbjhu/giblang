use core::panic;
use std::{collections::HashMap, fs};

use ariadne::Source;
use glob::glob;
use ptree::{item::StringItem, TreeBuilder};

use crate::{
    check::check_file,
    fs::tree_node::FileState,
    parser::{expr::qualified_name::SpannedQualifiedName, parse_file, top::impl_::Impl},
    util::Spanned,
};

use super::{
    export::{Export, MutExport},
    name::QualifiedName,
    tree_node::FileTreeNode,
};

pub struct Project {
    file_tree: FileTreeNode,
    impls: HashMap<QualifiedName, Impl>,
}

impl Project {
    pub fn init_pwd() -> Project {
        let mut project = Project::default();
        let files = glob("**/*.gib").unwrap();
        for file in files {
            let p = file.unwrap();
            let path_str = p.to_string_lossy();
            let path = path_str
                .strip_suffix(".gib")
                .unwrap()
                .split('/')
                .map(str::to_string)
                .collect::<QualifiedName>();

            let txt = fs::read_to_string(path_str.as_ref()).unwrap();
            let src = Source::from(txt.clone());
            project.insert(&path_str, &txt, &src, &path)
        }
        project
    }

    pub fn insert(&mut self, filename: &str, txt: &str, src: &Source, path: &[String]) {
        let ast = parse_file(txt, filename, src);
        let key = path.last().unwrap().clone();
        let module = &path[0..path.len() - 1];
        let current = self.file_tree.get_or_put_path(module);
        if let MutExport::Module(current) = current {
            if let FileTreeNode::Module(current) = current {
                current.insert(
                    key,
                    FileTreeNode::File(FileState {
                        text: txt.to_string(),
                        ast,
                        filename: filename.to_string(),
                    }),
                );
                return;
            }
        }
        panic!("Can't insert into non-module")
    }

    pub fn build_tree(&self) -> StringItem {
        let mut builder = TreeBuilder::new("/".to_string());
        self.file_tree.build_tree("/", &mut builder);
        builder.build()
    }

    pub fn get_path_with_error<'module>(
        &'module self,
        path: &SpannedQualifiedName,
    ) -> Result<Export<'module>, Spanned<String>> {
        self.file_tree.get_path_with_error(path)
    }

    pub fn check(&self) {
        self.file_tree.for_each(&|file| check_file(file, self))
    }
}

impl Default for Project {
    fn default() -> Self {
        Self {
            file_tree: Default::default(),
            impls: Default::default(),
        }
    }
}
