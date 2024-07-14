use std::collections::HashMap;

use ptree::TreeBuilder;

use crate::{
    fs::mut_export::MutExport,
    parser::{build_tree, top::Top, File},
    util::Spanned,
};

use super::{export::Export, name::QualifiedName};

pub enum FileTreeNode {
    File(FileState),
    Module(HashMap<String, FileTreeNode>),
}

pub struct FileState {
    pub text: String,
    pub ast: File,
    pub filename: String,
}

impl Default for FileTreeNode {
    fn default() -> Self {
        FileTreeNode::Module(HashMap::new())
    }
}

impl FileTreeNode {
    pub fn get<'module>(&'module self, name: &str) -> Option<Export<'module>> {
        match self {
            FileTreeNode::File(file) => get_top(&file.ast, name).map(|top| top.into()),
            FileTreeNode::Module(module) => module.get(name).map(Export::Module),
        }
    }

    pub fn get_mut<'module>(&'module mut self, name: &str) -> Option<MutExport<'module>> {
        match self {
            FileTreeNode::File(f) => get_top_mut(&mut f.ast, name).map(|top| top.into()),
            FileTreeNode::Module(module) => module.get_mut(name).map(MutExport::Module),
        }
    }

    pub fn get_path<'module>(&'module self, path: &[String]) -> Option<Export<'module>> {
        let mut current = Export::Module(self);
        for name in path {
            let new = current.get(name)?;
            current = new;
        }
        Some(current)
    }

    pub fn get_path_mut<'module>(&'module mut self, path: &[String]) -> Option<MutExport<'module>> {
        let mut current = MutExport::Module(self);
        for name in path {
            let new = current.get_mut(name)?;
            current = new;
        }
        Some(current)
    }

    pub fn get_or_put<'module>(&'module mut self, name: &str) -> MutExport<'module> {
        if self.has_key(name) {
            self.get_mut(name).unwrap()
        } else if let FileTreeNode::Module(module) = self {
            module.insert(name.to_string(), FileTreeNode::default());
            self.get_mut(name).unwrap()
        } else {
            panic!("Cannot insert into a file!")
        }
    }

    pub fn get_or_put_path<'module>(&'module mut self, path: &[String]) -> MutExport<'module> {
        let mut current = MutExport::Module(self);
        for name in path {
            let new = current.get_or_put(name);
            current = new;
        }
        current
    }

    pub fn has_key(&self, name: &str) -> bool {
        if let FileTreeNode::Module(module) = &self {
            module.contains_key(name)
        } else {
            false
        }
    }

    pub fn build_tree(&self, name: &str, builder: &mut TreeBuilder) {
        match self {
            FileTreeNode::File(file) => build_tree(file, name, builder),
            FileTreeNode::Module(module) => {
                builder.begin_child(name.to_string());
                for (name, item) in module {
                    item.build_tree(name, builder)
                }
                builder.end_child();
            }
        }
    }

    pub fn get_path_with_error<'module>(
        &'module self,
        path: &[Spanned<String>],
    ) -> Result<Export<'module>, Spanned<String>> {
        let mut current = Export::Module(self);
        for name in path {
            if let Some(new) = current.get(&name.0) {
                current = new;
            } else {
                return Err(name.clone());
            }
        }
        Ok(current)
    }

    pub fn get_path_with_error_mut<'module>(
        &'module mut self,
        path: &[Spanned<String>],
    ) -> Result<MutExport<'module>, Spanned<String>> {
        let mut current = MutExport::Module(self);
        for name in path {
            if let Some(new) = current.get_mut(&name.0) {
                current = new;
            } else {
                return Err(name.clone());
            }
        }
        Ok(current)
    }

    pub fn for_each<F: FnMut(&FileState)>(&self, f: &mut F) {
        match self {
            FileTreeNode::File(file) => f(file),
            FileTreeNode::Module(module) => module.values().for_each(|node| node.for_each(f)),
        }
    }
}

pub fn file_name_of(name: &QualifiedName) -> String {
    format!("{}.gib", name.join("/"))
}

fn get_top<'module>(file: &'module File, name: &str) -> Option<&'module Top> {
    file.iter()
        .find(|top| {
            if let Some(top) = top.0.get_name() {
                top == name
            } else {
                false
            }
        })
        .map(|top| &top.0)
}

fn get_top_mut<'module>(file: &'module mut File, name: &str) -> Option<&'module mut Top> {
    file.iter_mut()
        .find(|top| {
            if let Some(top) = top.0.get_name() {
                top == name
            } else {
                false
            }
        })
        .map(|top| &mut top.0)
}

impl<'module> From<&'module mut Top> for MutExport<'module> {
    fn from(value: &'module mut Top) -> Self {
        match value {
            Top::Func(f) => MutExport::Func(f),
            Top::Struct(s) => MutExport::Struct(s),
            Top::Enum(e) => MutExport::Enum(e),
            Top::Trait(t) => MutExport::Trait(t),
            Top::Impl(_) => panic!("Cannot convert 'impl' into export"),
            Top::Use(_) => panic!("Cannot convert 'use' into export"),
        }
    }
}
