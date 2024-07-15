use core::panic;
use std::{collections::HashMap, fs};

use ariadne::Source;
use chumsky::error::Rich;
use glob::glob;
use ptree::{item::StringItem, TreeBuilder};

use crate::{
    check::{
        check_file,
        impls::{build_impls, Impls},
        state::CheckState,
        ty::Ty,
    },
    cli::build::print_error,
    fs::{mut_export::MutExport, tree_node::FileState, util::path_from_filename},
    lexer::token::Token,
    parser::{parse_file, top::impl_::Impl},
    util::{Span, Spanned},
};

use super::{export::Export, name::QualifiedName, tree_node::FileTreeNode};

#[derive(Default)]
pub struct Project {
    file_tree: FileTreeNode,
}

impl Project {
    pub fn init_pwd() -> Project {
        let mut project = Project::default();
        let files = glob("**/*.gib").unwrap();
        let mut counter: u32 = 0;
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
            project.insert(&path_str, &txt, &src, &path, &mut counter)
        }
        project
    }

    pub fn insert(
        &mut self,
        filename: &str,
        txt: &str,
        src: &Source,
        path: &[String],
        counter: &mut u32,
    ) {
        let ast = parse_file(txt, filename, src, counter);
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
            .for_each(&mut |file| build_impls(file, self, &mut impls));
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

    pub fn get_file<'module>(&'module self, path: &[String]) -> &'module FileState {
        let found = self.file_tree.get_path(path);
        if let Some(Export::Module(FileTreeNode::File(data))) = found {
            data
        } else {
            panic!("FileData not found")
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ImplData {
    pub impl_: Impl,
    pub source: Source,
    pub filename: String,
    pub trait_path: Option<QualifiedName>,
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

    pub fn map<'module>(
        &'module self,
        ty: &Ty<'module>,
        project: &'module Project,
    ) -> Option<Ty<'module>> {
        let path = path_from_filename(&self.filename);
        let file = project.get_file(&path);
        let mut state = CheckState::from_file(file);
        let for_ = self.impl_.for_.0.check(project, &mut state, false);
        let trait_ = self.impl_.trait_.0.check(project, &mut state, false);
        let generics = for_.imply_generics(ty)?;
        if generics.len() == self.impl_.generics.0.len()
            && self
                .impl_
                .generics
                .0
                .iter()
                .all(|(arg, _)| generics.contains_key(&arg.name.0))
        {
            Some(trait_.parameterize(&generics))
        } else {
            None
        }
    }
}

impl<'module> Ty<'module> {
    pub fn imply_generics(&self, other: &Ty<'module>) -> Option<HashMap<String, Ty<'module>>> {
        match (self, other) {
            // TODO: Check use of variance/super
            (Ty::Generic { name, .. }, _) => {
                let mut res = HashMap::new();
                res.insert(name.to_string(), other.clone());
                return Some(res);
            }
            (
                Ty::Named { name, args },
                Ty::Named {
                    name: other_name,
                    args: other_args,
                },
            ) => {
                if name.id() == other_name.id() && args.len() == other_args.len() {
                    let mut res = HashMap::new();
                    for (s, o) in args.iter().zip(other_args) {
                        res.extend(s.imply_generics(o)?)
                    }
                    return Some(res);
                } else {
                    return None;
                }
            }
            _ => {}
        };

        if self.equals(other) {
            Some(HashMap::new())
        } else {
            None
        }
    }

    pub fn parameterize(&self, generics: &HashMap<String, Ty<'module>>) -> Ty<'module> {
        match self {
            Ty::Any => Ty::Any,
            Ty::Unknown => Ty::Unknown,
            Ty::Named { name, args } => Ty::Named {
                name: name.clone(),
                args: args.iter().map(|ty| ty.parameterize(generics)).collect(),
            },
            // TODO: Check use of variance/super
            Ty::Generic { name, .. } => {
                if let Some(ty) = generics.get(name) {
                    ty.clone()
                } else {
                    self.clone()
                }
            }
            Ty::Tuple(tys) => Ty::Tuple(tys.iter().map(|ty| ty.parameterize(generics)).collect()),
            Ty::Function {
                receiver,
                args,
                ret,
            } => {
                if let Some(receiver) = receiver {
                    Ty::Function {
                        receiver: Some(Box::new(receiver.parameterize(generics))),
                        args: args.iter().map(|ty| ty.parameterize(generics)).collect(),
                        ret: Box::new(ret.parameterize(generics)),
                    }
                } else {
                    Ty::Function {
                        receiver: None,
                        args: args.iter().map(|ty| ty.parameterize(generics)).collect(),
                        ret: Box::new(ret.parameterize(generics)),
                    }
                }
            }
            Ty::Prim(_) => self.clone(),
            Ty::Meta(_) => unimplemented!("Need to thing about this..."),
        }
    }
}
