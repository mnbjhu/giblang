use crate::{
    check::err::{simple::Simple, unresolved::Unresolved},
    project::Project,
};

pub mod simple;
pub mod unresolved;

pub enum CheckError {
    Simple(Simple),
    Unresolved(Unresolved),
}

impl Project {
    pub fn print_error(&self, error: &CheckError) {
        match error {
            CheckError::Simple(simple) => simple.print(self),
            CheckError::Unresolved(unresolved) => unresolved.print(self),
        }
        {}
    }
}
