use core::panic;

use crate::{
    parser::{
        common::generic_args::GenericArgs,
        top::{
            enum_::Enum, enum_member::EnumMember, func::Func, struct_::Struct, trait_::Trait, Top,
        },
    },
    util::Spanned,
};

use super::{project::ImplData, tree_node::FileTreeNode};

#[derive(Clone, Debug)]
pub enum Export<'module> {
    Func(&'module Func),
    Struct(&'module Struct),
    Trait(&'module Trait),
    Enum(&'module Enum),
    Member {
        parent: &'module Enum,
        member: &'module EnumMember,
    },
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
            return module.get_path_with_error(path);
        } else if let Export::Enum(e) = self {
            let member = e.members.iter().find(|m| m.0.name.0 == path[0].0);
            match member {
                Some(member) => {
                    return Export::Member {
                        parent: e,
                        member: &member.0,
                    }
                    .get_path_with_error(&path[1..])
                }
                None => return Err(path[0].clone()),
            };
        }
        Err(path[0].clone())
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
            Export::Member { .. } => panic!("Enum menber doesn't have generic args"),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Export::Struct(s) => &s.name.0,
            Export::Trait(t) => &t.name.0,
            Export::Enum(e) => &e.name.0,
            Export::Member { member, .. } => &member.name.0,
            Export::Func(Func { name, .. }) => &name.0,
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
