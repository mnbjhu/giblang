use crate::{
    check::state::CheckState,
    parser::top::{struct_body::StructBody, struct_field::StructField},
    project::decl::struct_::StructDecl,
    ty::Ty,
};

impl StructBody {
    pub fn resolve(&self, state: &mut CheckState) -> StructDecl {
        match self {
            StructBody::None => StructDecl::None,
            StructBody::Tuple(v) => {
                StructDecl::Tuple(v.iter().map(|f| f.0.resolve(state)).collect())
            }
            StructBody::Fields(fields) => {
                let mut new = Vec::new();
                for (field, _) in fields {
                    new.push(field.resolve(state));
                }
                StructDecl::Fields(new)
            }
        }
    }
}

impl StructField {
    pub fn resolve(&self, state: &mut CheckState) -> (String, Ty) {
        (self.name.0.to_string(), self.ty.0.resolve(state))
    }
}
