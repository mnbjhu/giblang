use crate::{check::state::VarDecl, db::decl::Decl, ty::Generic};

pub enum IdentDef<'db> {
    Variable(VarDecl<'db>),
    Generic(Generic<'db>),
    Decl(Decl<'db>),
}
