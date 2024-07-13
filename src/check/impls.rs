use std::collections::HashMap;

use ariadne::Source;

use crate::{
    fs::{name::QualifiedName, project::Project, tree_node::FileState},
    parser::top::{impl_::Impl, Top},
};

use super::CheckState;

pub fn build_impls(file: &FileState, project: &Project, impls: &mut Impls) {
    let source = &Source::from(file.text.clone());
    let mut state = CheckState::new(&file.filename, source);
    for (item, _) in &file.ast {
        match item {
            Top::Use(use_) => {
                state.import(use_, project, true);
            }
            Top::Impl(impl_) => {
                let name = impl_
                    .for_
                    .0
                    .name
                    .iter()
                    .map(|(name, _)| name.to_string())
                    .collect::<QualifiedName>();
                impls.insert(&name, impl_.clone());
            }
            _ => {}
        }
    }
}

#[derive(Default)]
pub struct Impls(pub HashMap<QualifiedName, Vec<Impl>>);

impl Impls {
    pub fn insert(&mut self, name: &QualifiedName, impl_: Impl) {
        if let Some(existing) = self.0.get_mut(name) {
            existing.push(impl_);
        } else {
            self.0.insert(name.clone(), vec![impl_]);
        }
    }

    pub fn get(&self, name: &QualifiedName) -> Option<&Vec<Impl>> {
        self.0.get(name)
    }
}
