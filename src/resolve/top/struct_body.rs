use crate::{
    db::decl::struct_::StructDecl, parser::top::{struct_body::StructBody, struct_field::StructField}, resolve::state::ResolveState, ty::Ty
};

impl StructBody {
    pub fn resolve<'db>(&self, state: &mut ResolveState<'db>) -> StructDecl<'db> {
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
    pub fn resolve<'db>(&self, state: &mut ResolveState<'db>) -> (String, Ty<'db>) {
        (self.name.0.to_string(), self.ty.0.resolve(state))
    }
}
