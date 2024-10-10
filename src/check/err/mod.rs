use impl_type::ImplTypeMismatch;
use is_not_instance::IsNotInstance;
use missing_receiver::MissingReceiver;
use unexpected_args::UnexpectedArgs;
use unresolved_type_var::UnboundTypeVar;
use wildcard::UnexpectedWildcard;

use crate::{
    check::err::{simple::Simple, unresolved::Unresolved},
    db::input::Db,
};

pub mod impl_type;
pub mod is_not_instance;
pub mod missing_receiver;
pub mod simple;
pub mod unexpected_args;
pub mod unresolved;
pub mod unresolved_type_var;
pub mod wildcard;

#[salsa::accumulator]
pub struct Error {
    pub inner: CheckError,
}

#[derive(Clone, Debug)]
pub enum CheckError {
    Simple(Simple),
    Unresolved(Unresolved),
    IsNotInstance(IsNotInstance),
    UnboundTypeVar(UnboundTypeVar),
    UnexpectedArgs(UnexpectedArgs),
    MissingReceiver(MissingReceiver),
    UnexpectedWildcard(UnexpectedWildcard),
    ImplTypeMismatch(ImplTypeMismatch),
}

impl CheckError {
    pub fn print(&self, db: &dyn Db) {
        match self {
            CheckError::Simple(err) => err.print(db),
            CheckError::Unresolved(err) => err.print(db),
            CheckError::IsNotInstance(err) => err.print(db),
            CheckError::UnboundTypeVar(err) => err.print(db),
            CheckError::UnexpectedArgs(err) => err.print(db),
            CheckError::MissingReceiver(err) => err.print(db),
            CheckError::UnexpectedWildcard(err) => err.print(db),
            CheckError::ImplTypeMismatch(err) => err.print(db),
        }
    }
}
