
use crate::{
    check::state::CheckState,
    ty::{prim::PrimTy, Generic, Ty},
    util::Spanned,
};

use self::struct_::StructDecl;

pub mod impl_;
pub mod struct_;

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
    #[must_use]
    pub fn generics(&self) -> Vec<Generic> {
        match self {
            Decl::Struct { generics, .. }
            | Decl::Trait { generics, .. }
            | Decl::Enum { generics, .. }
            | Decl::Function { generics, .. } => generics.clone(),
            Decl::Member { .. } => {
                panic!("Hmm, don't think I need this, guess I'll find out")
            }
            Decl::Prim(_) => vec![],
        }
    }

    #[must_use]
    pub fn name(&self) -> String {
        match self {
            Decl::Struct { name, .. }
            | Decl::Trait { name, .. }
            | Decl::Enum { name, .. }
            | Decl::Function { name, .. }
            | Decl::Member { name, .. } => name.0.clone(),
            Decl::Prim(p) => p.to_string(),
        }
    }

    pub fn get_ty(&self, id: u32, state: &mut CheckState) -> Ty {
        match self {
            Decl::Trait { .. } => todo!(),
            Decl::Enum { .. } => todo!(),
            Decl::Member { body, .. } | Decl::Struct { body, .. } => {
                let self_ty = self.get_named_ty(state, id);
                body.get_constructor_ty(self_ty)
            }
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

    fn get_named_ty(&self, state: &mut CheckState, id: u32) -> Ty {
        if let Decl::Member { .. } = &self {
            let parent = state
                .project
                .get_parent(id)
                .expect("Member decls should have a parent");
            let parent_decl = state.project.get_decl(parent);
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
