use ariadne::Source;

use crate::{
    fs::{
        name::QualifiedName,
        project::{ImplData, Project},
        tree_node::FileState,
    },
    parser::top::Top,
};

use super::{CheckState, NamedExpr};

pub fn build_impls(file: &FileState, project: &Project, impls: &mut Impls) {
    let source = &Source::from(file.text.clone());
    let mut state = CheckState::from_file(file, project);
    for (item, _) in &file.ast {
        match item {
            Top::Use(use_) => {
                state.import(use_, project, true);
            }
            Top::Impl(impl_) => {
                let for_path = state.get_path(&impl_.for_.0.name, project, true);
                let trait_path = state.get_path(&impl_.trait_.0.name, project, true);
                let trait_path = if let NamedExpr::Imported(_, p) = trait_path {
                    Some(p)
                } else {
                    None
                };
                if let NamedExpr::Imported(_, p) = for_path {
                    impls.insert(
                        p,
                        ImplData {
                            impl_: impl_.clone(),
                            source: source.clone(),
                            filename: file.filename.to_string(),
                            trait_path,
                        },
                    );
                }
            }
            _ => {}
        }
    }
}

#[derive(Default)]
pub struct Impls(pub Vec<(QualifiedName, ImplData)>);

impl Impls {
    pub fn insert(&mut self, name: QualifiedName, impl_: ImplData) {
        self.0.push((name, impl_));
    }
}
