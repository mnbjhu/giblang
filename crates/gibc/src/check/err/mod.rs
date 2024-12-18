use impl_type::ImplTypeMismatch;
use is_not_instance::IsNotInstance;
use missing_receiver::MissingReceiver;
use unexpected_args::UnexpectedArgs;
use unresolved_type_var::UnboundTypeVar;
use wildcard::UnexpectedWildcard;

use crate::{
    check::err::{simple::Simple, unresolved::Unresolved},
    db::{err::Diagnostic, input::Db},
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
    #[allow(dead_code)]
    UnboundTypeVar(UnboundTypeVar),
    UnexpectedArgs(UnexpectedArgs),
    MissingReceiver(MissingReceiver),
    UnexpectedWildcard(UnexpectedWildcard),
    #[allow(dead_code)]
    ImplTypeMismatch(ImplTypeMismatch),
}

pub trait IntoWithDb<T> {
    fn into_with_db(self, db: &dyn Db) -> T;
}

impl IntoWithDb<Diagnostic> for Error {
    fn into_with_db(self, db: &dyn Db) -> Diagnostic {
        match self.inner {
            CheckError::Simple(err) => err.into_with_db(db),
            CheckError::Unresolved(err) => err.into_with_db(db),
            CheckError::IsNotInstance(err) => err.into_with_db(db),
            CheckError::UnboundTypeVar(err) => err.into_with_db(db),
            CheckError::UnexpectedArgs(err) => err.into_with_db(db),
            CheckError::MissingReceiver(err) => err.into_with_db(db),
            CheckError::UnexpectedWildcard(err) => err.into_with_db(db),
            CheckError::ImplTypeMismatch(err) => err.into_with_db(db),
        }
    }
}
