use core::panic;

use crate::{
    parser::{
        common::generic_args::GenericArgs,
        top::{enum_::Enum, func::Func, struct_::Struct, trait_::Trait, Top},
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
        if path.is_empty() {
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

impl<'module> From<&'module Top> for Export<'module> {
    fn from(value: &'module Top) -> Self {
        match value {
            Top::Func(f) => Export::Func(f),
            Top::Struct(s) => Export::Struct(s),
            Top::Enum(e) => Export::Enum(e),
            Top::Trait(t) => Export::Trait(t),
            Top::Impl(_) => panic!("Cannot convert 'impl' into export"),
            Top::Use(_) => panic!("Cannot convert 'use' into export"),
        }
    }
}
