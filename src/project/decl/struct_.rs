use crate::ty::{FuncTy, Ty};

#[derive(Debug)]
pub enum StructDecl {
    Fields(Vec<(String, Ty)>),
    Tuple(Vec<Ty>),
    None,
}

impl StructDecl {
    #[must_use]
    pub fn get_constructor_ty(&self, self_ty: Ty) -> Ty {
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
