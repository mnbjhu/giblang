use ariadne::Source;

use crate::{
    fs::{
        name::QualifiedName,
        project::{ImplData, Project},
        tree_node::FileState,
    },
    parser::top::Top,
};

use super::{path_from_filename, ty::PrimTy, CheckState, NamedExpr};

pub fn build_impls(file: &FileState, project: &Project, impls: &mut Impls) {
    let source = &Source::from(file.text.clone());
    let mut state = CheckState::new(&file.filename, source.clone());
    state.insert("String".to_string(), NamedExpr::Prim(PrimTy::String));
    state.insert("Bool".to_string(), NamedExpr::Prim(PrimTy::Bool));
    state.insert("Float".to_string(), NamedExpr::Prim(PrimTy::Float));
    state.insert("Int".to_string(), NamedExpr::Prim(PrimTy::Int));
    for (top, _) in &file.ast {
        if let Some(name) = top.get_name() {
            let mut path = path_from_filename(&file.filename);
            path.push(name.to_string());
            state.insert(
                name.to_string(),
                NamedExpr::Imported(project.from(top), path),
            )
        }
    }
    for (item, _) in &file.ast {
        match item {
            Top::Use(use_) => {
                state.import(use_, project, true);
            }
            Top::Impl(impl_) => {
                let found = state.get_path(&impl_.for_.0.name, project, true);
                if let NamedExpr::Imported(_, p) = found {
                    impls.insert(
                        p,
                        ImplData {
                            impl_: impl_.clone(),
                            source: source.clone(),
                            filename: file.filename.to_string(),
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
        let text = name.join("::");
        println!("Adding to 'Impls' {text}");
        self.0.push((name, impl_));
    }
}
