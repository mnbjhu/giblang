use core::panic;
use std::collections::HashMap;

use crate::{
    check::state::CheckState,
    parser::top::Top,
    project::{ImplData, Project},
    ty::{Generic, PrimTy, Ty},
    util::Spanned,
};

pub mod enum_;
pub mod enum_member;
pub mod func;
pub mod func_arg;
pub mod impl_;
pub mod member;
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
                        state.error("The 'for' of an 'impl' should a named type", i.for_.1);
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

#[derive(Debug)]
pub enum Decl {
    Struct {
        name: Spanned<String>,
        generics: Vec<Generic>,
        body: StructDecl,
    },
    Trait {
        name: Spanned<String>,
        generics: Vec<Generic>,
        body: Vec<u32>,
    },
    Enum {
        name: Spanned<String>,
        generics: Vec<Generic>,
        variants: Vec<u32>,
    },
    Member {
        name: Spanned<String>,
        body: StructDecl,
    },
    Function {
        name: Spanned<String>,
        generics: Vec<Generic>,
        receiver: Option<Ty>,
        args: Vec<(String, Ty)>,
        ret: Ty,
    },
    Prim(PrimTy),
}

impl Decl {
    pub fn generics(&self) -> Vec<Generic> {
        match self {
            Decl::Struct { generics, .. } => generics.clone(),
            Decl::Trait { generics, .. } => generics.clone(),
            Decl::Enum { generics, .. } => generics.clone(),
            Decl::Function { generics, .. } => generics.clone(),
            Decl::Member { .. } => {
                panic!("Hmm, don't think I need this, guess I'll find out")
            }
            Decl::Prim(_) => vec![],
        }
    }

    pub fn name(&self) -> String {
        match self {
            Decl::Struct { name, .. } => name.0.clone(),
            Decl::Trait { name, .. } => name.0.clone(),
            Decl::Enum { name, .. } => name.0.clone(),
            Decl::Function { name, .. } => name.0.clone(),
            Decl::Member { name, .. } => name.0.clone(),
            Decl::Prim(p) => p.to_string(),
        }
    }

    pub fn get_ty(&self, id: u32, project: &Project) -> Ty {
        let self_ty = self.get_named_ty(project, id);
        match self {
            Decl::Struct { body, .. } => body.get_constructor_ty(self_ty),
            Decl::Trait { .. } => todo!(),
            Decl::Enum { .. } => todo!(),
            Decl::Member { body, .. } => body.get_constructor_ty(self_ty),
            Decl::Function {
                receiver,
                args,
                ret,
                ..
            } => {
                let args = args.iter().map(|(_, ty)| ty.clone()).collect::<Vec<_>>();
                Ty::Function {
                    receiver: receiver.clone().map(Box::new),
                    args,
                    ret: Box::new(ret.clone()),
                }
            }
            Decl::Prim(p) => Ty::Meta(Box::new(p.into())),
        }
    }

    fn get_named_ty(&self, project: &Project, id: u32) -> Ty {
        if let Decl::Member { .. } = &self {
            let parent = project
                .get_parent(id)
                .expect("Member decls should have a parent");
            let parent_decl = project.get_decl(parent);
            Ty::Named {
                name: parent,
                args: parent_decl
                    .generics()
                    .iter()
                    .cloned()
                    .map(Ty::Generic)
                    .collect(),
            }
        } else {
            Ty::Named {
                name: id,
                args: self.generics().iter().cloned().map(Ty::Generic).collect(),
            }
        }
    }
}

#[derive(Debug)]
pub enum StructDecl {
    Fields(Vec<(String, Ty)>),
    Tuple(Vec<Ty>),
    None,
}

impl StructDecl {
    pub fn get_constructor_ty(&self, self_ty: Ty) -> Ty {
        match self {
            StructDecl::Fields(fields) => {
                let args = fields.iter().map(|(_, ty)| ty.clone()).collect();
                Ty::Function {
                    receiver: None,
                    args,
                    ret: Box::new(self_ty),
                }
            }
            StructDecl::Tuple(fields) => {
                let args = fields.to_vec();
                Ty::Function {
                    receiver: None,
                    args,
                    ret: Box::new(self_ty),
                }
            }
            StructDecl::None => Ty::Unknown,
        }
    }
}
