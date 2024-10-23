use salsa::Update;

use crate::{
    db::path::ModulePath, parser::common::variance::Variance, util::Spanned
};

pub mod func;
pub mod imply;
pub mod is_instance;
pub mod name;
pub mod parameterize;
pub mod inst;


#[derive(Clone, Debug, PartialEq, Default, Update, Hash, Eq)]
pub enum Ty<'db> {
    Any,
    #[default]
    Unknown,
    Named {
        name: ModulePath<'db>,
        args: Vec<Ty<'db>>,
    },
    TypeVar {
        id: u32,
    },
    Generic(Generic<'db>),
    Meta(Box<Ty<'db>>),
    Function(FuncTy<'db>),
    Tuple(Vec<Ty<'db>>),
    Sum(Vec<Ty<'db>>),
    Nothing,
}

#[derive(Clone, Debug, Eq, PartialEq, Update, Hash)]
pub struct Generic<'db> {
    pub name: Spanned<String>,
    pub variance: Variance,
    pub super_: Box<Ty<'db>>,
}

#[derive(Clone, Debug, PartialEq, Default, Update, Eq, Hash)]
pub struct FuncTy<'db> {
    pub receiver: Option<Box<Ty<'db>>>,
    pub args: Vec<Ty<'db>>,
    pub ret: Box<Ty<'db>>,
}


impl<'db> Ty<'db> {
    pub fn is_unit(&self) -> bool {
        if let Ty::Tuple(tys) = self {
            tys.is_empty()
        } else {
            false
        }
    }
}
