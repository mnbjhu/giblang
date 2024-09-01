use crate::{
    check::state::CheckState,
    parser::{common::type_::Type, top::struct_field::StructField},
    ty::Ty,
};

impl StructField {
    pub fn build(&self, state: &mut CheckState) -> String {
        let ty_txt = build_field_ty(&self.ty.0, state);
        format!("{} {ty_txt}\n", self.name.0)
    }
}
pub fn build_field_ty(ty: &Type, state: &mut CheckState) -> String {
    let ty = ty.check(state);
    let mut ty_txt = ty.build(state);
    if let Ty::Named { name: ..5, .. } = &ty {
    } else {
        ty_txt.insert(0, '*');
    }
    ty_txt
}
