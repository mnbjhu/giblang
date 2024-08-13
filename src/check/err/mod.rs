use impl_type::ImplTypeMismatch;
use is_not_instance::IsNotInstance;
use missing_receiver::MissingReceiver;
use unexpected_args::UnexpectedArgs;
use unresolved_type_var::UnboundTypeVar;
use wildcard::UnexpectedWildcard;

use crate::{
    check::err::{simple::Simple, unresolved::Unresolved},
    project::Project,
};

use super::state::CheckState;

pub mod impl_type;
pub mod is_not_instance;
pub mod missing_receiver;
pub mod simple;
pub mod unexpected_args;
pub mod unresolved;
pub mod unresolved_type_var;
pub mod wildcard;

#[derive(Clone, Debug, PartialEq)]
pub enum CheckError {
    Simple(Simple),
    Unresolved(Unresolved),
    IsNotInstance(IsNotInstance),
    UnboundTypeVar(UnboundTypeVar),
    UnexpectedArgs(UnexpectedArgs),
    MissingReceiver(MissingReceiver),
}

pub enum ResolveError {
    Unresolved(Unresolved),
    UnexpectedWildcard(UnexpectedWildcard),
    ImplTypeMismatch(ImplTypeMismatch),
}
impl CheckState<'_> {
    pub fn print_error(&self, error: &CheckError) {
        match error {
            CheckError::Simple(e) => e.print(self.project),
            CheckError::Unresolved(e) => e.print(self.project),
            CheckError::IsNotInstance(e) => e.print(self),
            CheckError::UnboundTypeVar(e) => e.print(self),
            CheckError::MissingReceiver(e) => e.print(self),
            CheckError::UnexpectedArgs(e) => e.print(self),
        }
    }
}

impl Project {
    pub fn print_resolve_error(&self, error: &ResolveError) {
        match error {
            ResolveError::Unresolved(e) => e.print(self),
            ResolveError::UnexpectedWildcard(e) => e.print(self),
            ResolveError::ImplTypeMismatch(e) => e.print(self),
        }
    }
}
