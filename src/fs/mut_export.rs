use crate::{
    fs::{project::ImplData, tree_node::FileTreeNode},
    parser::top::{enum_::Enum, func::Func, struct_::Struct, trait_::Trait},
};

pub enum MutExport<'module> {
    Func(&'module mut Func),
    Struct(&'module mut Struct),
    Trait(&'module mut Trait),
    Enum(&'module mut Enum),
    Module(&'module mut FileTreeNode),
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
