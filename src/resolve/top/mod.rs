use std::collections::HashMap;

use crate::{
    check::state::CheckState,
    parser::top::Top,
    project::{decl::Decl, ImplData},
    ty::Ty,
};

pub mod enum_;
pub mod enum_member;
pub mod func;
pub mod func_arg;
pub mod impl_;
pub mod struct_;
pub mod struct_body;
pub mod trait_;

impl Top {
    pub fn resolve(
        &self,
        state: &mut CheckState,
        decls: &mut HashMap<u32, Decl>,
        impls: &mut HashMap<u32, ImplData>,
        impl_map: &mut HashMap<u32, Vec<u32>>,
    ) {
        if let Top::Use(use_) = self {
            state.import(use_)
        } else {
            let id = self.get_id().unwrap();
            let decl = match self {
                Top::Func(f) => f.resolve(state),
                Top::Struct(s) => s.resolve(state),
                Top::Enum(e) => e.resolve(state, decls),
                Top::Trait(t) => t.resolve(state, decls),
                Top::Impl(i) => {
                    let id = i.id;
                    let impl_ = i.resolve(state, decls);
                    if let Ty::Named { name, .. } = &impl_.from {
                        if let Some(existing) = impl_map.get_mut(name) {
                            existing.push(id);
                        } else {
                            impl_map.insert(*name, vec![id]);
                        }
                    } else {
                        state.simple_error("The 'for' of an 'impl' should a named type", i.for_.1);
                    };
                    impls.insert(id, impl_);
                    return;
                }
                Top::Use(_) => todo!(),
            };
            decls.insert(id, decl);
        }
    }
}
