use salsa::Update;

use crate::{check::state::VarDecl, db::decl::Decl, ty::Generic};

#[derive(Debug, PartialEq, Clone, Update, Eq)]
pub enum IdentDef<'db> {
    Variable(VarDecl<'db>),
    Generic(Generic<'db>),
    Decl(Decl<'db>),
    Unresolved,
}
