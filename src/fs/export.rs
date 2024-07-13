use core::panic;

use crate::{
    parser::top::{enum_::Enum, func::Func, struct_::Struct, trait_::Trait},
    util::Spanned,
};

use super::tree_node::FileTreeNode;

#[derive(Clone)]
pub enum Export<'module> {
    Func(&'module Func),
    Struct(&'module Struct),
    Trait(&'module Trait),
    Enum(&'module Enum),
    Module(&'module FileTreeNode),
}

pub enum MutExport<'module> {
    Func(&'module mut Func),
    Struct(&'module mut Struct),
    Trait(&'module mut Trait),
    Enum(&'module mut Enum),
    Module(&'module mut FileTreeNode),
}

impl<'module> Export<'module> {
    pub fn get(self, name: &str) -> Option<Export<'module>> {
        if let Export::Module(module) = self {
            module.get(name)
        } else {
            None
        }
    }

    pub fn valid_type(&self) -> bool {
        matches!(self, Export::Trait(_) | Export::Enum(_) | Export::Struct(_))
    }

    pub fn get_path_with_error(
        self,
        path: &[Spanned<String>],
    ) -> Result<Export<'module>, Spanned<String>> {
        if path.len() == 0 {
            return Ok(self);
        }
        if let Export::Module(module) = self {
            module.get_path_with_error(path)
        } else {
            panic!("Cannot get from non-module")
        }
    }
}

impl<'module> MutExport<'module> {
    pub fn get_mut(self, name: &str) -> Option<MutExport<'module>> {
        if let MutExport::Module(module) = self {
            module.get_mut(name)
        } else {
            None
        }
    }

    pub fn get_or_put(self, name: &str) -> MutExport<'module> {
        if let MutExport::Module(module) = self {
            module.get_or_put(name)
        } else {
            panic!("Cannot put in to non-module")
        }
    }
}
