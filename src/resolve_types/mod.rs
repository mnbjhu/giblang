use state::TypeResolveState;

use crate::parser::File;

mod common;
mod state;
mod stmt;
mod top;
mod ty;

pub fn type_resolve_file(file: File, state: &mut TypeResolveState) {
    for decl in file {
        decl.0.type_resolve(state);
    }
}
