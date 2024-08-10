use impl_type::ImplTypeMismatch;
use wildcard::UnexpectedWildcard;

use crate::{
    check::err::{simple::Simple, unresolved::Unresolved},
    project::Project,
};

pub mod impl_type;
pub mod simple;
pub mod unresolved;
pub mod wildcard;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CheckError {
    Simple(Simple),
    Unresolved(Unresolved),
}

pub enum ResolveError {
    Unresolved(Unresolved),
    UnexpectedWildcard(UnexpectedWildcard),
    ImplTypeMismatch(ImplTypeMismatch),
}

impl Project {
    pub fn print_error(&self, error: &CheckError) {
        match error {
            CheckError::Simple(simple) => simple.print(self),
            CheckError::Unresolved(unresolved) => unresolved.print(self),
        }
    }

    pub fn print_resolve_error(&self, error: &ResolveError) {
        match error {
            ResolveError::Unresolved(e) => e.print(self),
            ResolveError::UnexpectedWildcard(e) => e.print(self),
            ResolveError::ImplTypeMismatch(e) => e.print(self),
        }
    }
}
