use core::panic;
use std::fs;

use ariadne::Source;
use chumsky::error::Rich;
use glob::glob;
use ptree::{item::StringItem, TreeBuilder};

use crate::{
    check::{
        check_file,
        impls::{build_impls, Impls},
        CheckState,
    },
    cli::build::print_error,
    fs::tree_node::FileState,
    lexer::token::Token,
    parser::{
        parse_file,
        top::{impl_::Impl, Top},
    },
    util::{Span, Spanned},
};

use super::{
    export::{Export, MutExport},
    name::QualifiedName,
    tree_node::FileTreeNode,
};

#[derive(Default)]
pub struct Project {
    file_tree: FileTreeNode,
}

impl Project {
    pub fn from<'module>(&'module self, value: &'module Top) -> Export<'module> {
        self.file_tree.from(value)
    }

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
        if let MutExport::Module(FileTreeNode::Module(current)) = current {
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
        panic!("Can't insert into non-module")
    }

    pub fn build_tree(&self) -> StringItem {
        let mut builder = TreeBuilder::new("/".to_string());
        self.file_tree.build_tree("/", &mut builder);
        builder.build()
    }

    pub fn get_path_with_error<'module>(
        &'module self,
        path: &[Spanned<String>],
    ) -> Result<Export<'module>, Spanned<String>> {
        self.file_tree.get_path_with_error(path)
    }

    pub fn get_path_with_error_mut<'module>(
        &'module mut self,
        path: &[Spanned<String>],
    ) -> Result<MutExport<'module>, Spanned<String>> {
        self.file_tree.get_path_with_error_mut(path)
    }

    pub fn build_impls(&mut self) {
        let mut impls = Impls::default();
        self.file_tree
            .for_each(&mut |file| build_impls(file, &self, &mut impls));
        for (path, impl_) in impls.0 {
            self.insert_impl(&path, impl_)
        }
    }

    pub fn check(&self) {
        self.file_tree.for_each(&mut |file| check_file(file, self))
    }

    pub fn insert_impl(&mut self, path: &[String], impl_: ImplData) {
        let export = self.file_tree.get_path_mut(path);
        if let Some(export) = export {
            if let Some(impls) = export.impls_mut() {
                impls.push(impl_)
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ImplData {
    pub impl_: Impl,
    pub source: Source,
    pub filename: String,
}

impl ImplData {
    pub fn error(&self, msg: &str, span: Span) {
        print_error::<Token>(
            Rich::custom(span, msg),
            self.source.clone(),
            &self.filename,
            "ImplResolve",
        )
    }
}
