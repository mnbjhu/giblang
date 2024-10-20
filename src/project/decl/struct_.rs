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
    pub fn get_constructor_ty(&self, self_ty: Ty<'db>) -> Option<FuncTy<'db>> {
        match self {
            StructDecl::Fields(fields) => {
                let args = fields.iter().map(|(_, ty)| ty.clone()).collect();
                Some(FuncTy {
                    receiver: None,
                    args,
                    ret: Box::new(self_ty),
                })
            }
            StructDecl::Tuple(fields) => {
                let args = fields.clone();
                Some(FuncTy {
                    receiver: None,
                    args,
                    ret: Box::new(self_ty),
                })
            }
            StructDecl::None => None,
        }
    }

    #[must_use]
    pub fn is_none(&self) -> bool {
        matches!(self, StructDecl::None)
    }
}
