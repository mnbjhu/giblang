use std::collections::HashMap;

use crate::{
    check::state::CheckState,
    project::{decl::Decl, file_data::FileData, ImplData, Project},
};

mod common;
mod impl_;
mod top;

pub fn resolve_file(
    file_data: &FileData,
    decls: &mut HashMap<u32, Decl>,
    impls: &mut HashMap<u32, ImplData>,
    impl_map: &mut HashMap<u32, Vec<u32>>,
    project: &Project,
) {
    let mut state = CheckState::from_file(file_data, project);
    for (item, _) in &file_data.ast {
        state.enter_scope();
        item.resolve(&mut state, decls, impls, impl_map);
        state.exit_scope();
    }
}
