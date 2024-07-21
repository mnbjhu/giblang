use core::panic;
use std::collections::HashMap;

use crate::{
    parser::{
        common::{generic_arg::GenericArg, variance::Variance},
        top::Top,
    },
    project::Project,
    ty::Ty,
    util::Spanned,
};

pub mod enum_;
pub mod impl_;
pub mod member;
pub mod struct_;
pub mod trait_;

impl Top {
    pub fn resolve(&self, project: &mut Project) -> Decl {
        match self {
            Top::Func(_) => todo!(),
            Top::Struct(_) => todo!(),
            Top::Enum(_) => todo!(),
            Top::Trait(_) => todo!(),
            Top::Impl(_) => todo!(),
            Top::Use(_) => todo!(),
        }
    }
}

pub struct GenericArgDecl {
    pub name: String,
    pub variance: Variance,
    pub super_: Box<Ty>,
}

pub enum Decl {
    Struct {
        name: Spanned<String>,
        generics: Vec<GenericArgDecl>,
        body: StructDecl,
    },
    Trait {
        name: Spanned<String>,
        generics: Vec<GenericArgDecl>,
        body: StructDecl,
    },
    Enum {
        name: Spanned<String>,
        generics: Vec<GenericArgDecl>,
        variants: HashMap<String, Vec<Ty>>,
    },
    Member {
        name: Spanned<String>,
        body: StructDecl,
    },
    Function {
        name: Spanned<String>,
        generics: Vec<GenericArgDecl>,
        receiver: Option<Ty>,
        args: Vec<(String, Ty)>,
        ret: Ty,
    },
}

impl Decl {
    pub fn generics(&self) -> &[GenericArgDecl] {
        match self {
            Decl::Struct { generics, .. } => generics,
            Decl::Trait { generics, .. } => generics,
            Decl::Enum { generics, .. } => generics,
            Decl::Function { generics, .. } => generics,
            Decl::Member { name, body } => {
                panic!("Hmm, don't think I need this, guess I'll find out")
            }
        }
    }
}

pub enum StructDecl {
    Fields(Vec<(String, Ty)>),
    Tuple(Vec<Ty>),
    None,
}
