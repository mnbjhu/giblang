use core::panic;

use crate::parser::top::{enum_::Enum, func::Func, struct_::Struct, trait_::Trait};

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
