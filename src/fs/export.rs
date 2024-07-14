use core::panic;

use crate::{
    parser::{
        common::generic_args::GenericArgs,
        top::{enum_::Enum, func::Func, struct_::Struct, trait_::Trait},
    },
    util::Spanned,
};

use super::{project::ImplData, tree_node::FileTreeNode};

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

    pub fn impls(&self) -> Option<&'module Vec<ImplData>> {
        match self {
            Export::Struct(s) => Some(&s.impls),
            Export::Enum(e) => Some(&e.impls),
            Export::Trait(t) => Some(&t.impls),
            _ => None,
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

    pub fn id(&self) -> u32 {
        match self {
            Export::Struct(s) => s.id,
            Export::Trait(t) => t.id,
            Export::Enum(e) => e.id,
            _ => todo!(),
        }
    }

    pub fn generic_args(&'module self) -> &'module GenericArgs {
        match self {
            Export::Func(f) => &f.generics,
            Export::Struct(f) => &f.generics.0,
            Export::Enum(f) => &f.generics.0,
            Export::Trait(f) => &f.generics,
            Export::Module(_) => panic!("Module doesn't have generic args"),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Export::Struct(s) => &s.name.0,
            Export::Trait(t) => &t.name.0,
            Export::Enum(e) => &e.name.0,
            _ => unimplemented!(),
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

    pub fn impls_mut(self) -> Option<&'module mut Vec<ImplData>> {
        match self {
            MutExport::Struct(s) => Some(&mut s.impls),
            MutExport::Enum(e) => Some(&mut e.impls),
            MutExport::Trait(t) => Some(&mut t.impls),
            _ => None,
        }
    }
}
