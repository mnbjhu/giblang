use salsa::Update;

use crate::ty::{FuncTy, Ty};

#[derive(Update, Debug, Clone, PartialEq)]
pub enum StructDecl<'db> {
    Fields(Vec<(String, Ty<'db>)>),
    Tuple(Vec<Ty<'db>>),
    None,
}

impl<'db> StructDecl<'db> {
    #[must_use]
    pub fn get_constructor_ty(&self, self_ty: Ty<'db>) -> Ty<'db> {
        match self {
            StructDecl::Fields(fields) => {
                let args = fields.iter().map(|(_, ty)| ty.clone()).collect();
                Ty::Function(FuncTy {
                    receiver: None,
                    args,
                    ret: Box::new(self_ty),
                })
            }
            StructDecl::Tuple(fields) => {
                let args = fields.clone();
                Ty::Function(FuncTy {
                    receiver: None,
                    args,
                    ret: Box::new(self_ty),
                })
            }
            StructDecl::None => self_ty,
        }
    }

    #[must_use]
    pub fn is_none(&self) -> bool {
        matches!(self, StructDecl::None)
    }
}
