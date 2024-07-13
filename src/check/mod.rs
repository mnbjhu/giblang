use std::collections::HashMap;

use ariadne::Source;
use chumsky::error::Rich;

use crate::{
    cli::build::print_error,
    fs::{export::Export, project::Project, tree_node::FileState},
    lexer::token::Token,
    parser::{common::variance::Variance, expr::qualified_name::SpannedQualifiedName},
    util::{Span, Spanned},
};

use self::ty::{PrimTy, Ty};

pub mod common;
pub mod impls;
pub mod top;
pub mod ty;

pub struct CheckState<'module> {
    stack: Vec<HashMap<String, NamedExpr<'module>>>,
    file_name: &'module str,
    source: &'module Source,
}

#[derive(Clone, Default)]
pub enum NamedExpr<'module> {
    Export(Export<'module>),
    Variable(Ty<'module>),
    GenericArg {
        super_: Ty<'module>,
        variance: Variance,
    },
    Prim(PrimTy),
    #[default]
    Unknown,
}

impl<'module> CheckState<'module> {
    pub fn enter_scope(&mut self) {
        self.stack.push(HashMap::new())
    }

    pub fn exit_scope(&mut self) {
        self.stack.pop();
    }

    fn get_name(&self, name: &str) -> NamedExpr<'module> {
        for scope in self.stack.iter().rev() {
            if let Some(found) = scope.get(name) {
                return found.clone();
            }
        }
        NamedExpr::Unknown
    }

    pub fn get_path(
        &self,
        path: &[Spanned<String>],
        project: &'module Project,
        print_errors: bool,
    ) -> NamedExpr<'module> {
        let named = self.get_name(&path.first().unwrap().0);
        match named {
            NamedExpr::Unknown => {
                let found = project.get_path_with_error(path);
                match found {
                    Ok(found) => NamedExpr::Export(found),
                    Err((name, span)) => {
                        if print_errors {
                            self.error(&format!("Import not found '{name}'"), span);
                        }
                        NamedExpr::Unknown
                    }
                }
            }
            NamedExpr::Export(e) => {
                let found = e.get_path_with_error(&path[1..path.len()]);
                match found {
                    Ok(found) => NamedExpr::Export(found),
                    Err((name, span)) => {
                        if print_errors {
                            self.error(&format!("Import not found '{name}'"), span);
                        }
                        NamedExpr::Unknown
                    }
                }
            }
            _ => named,
        }
    }

    pub fn insert(&mut self, name: String, export: NamedExpr<'module>) {
        self.stack
            .last_mut()
            .expect("Check stack overflowed")
            .insert(name, export);
    }

    pub fn new(file_name: &'module str, source: &'module Source) -> Self {
        Self {
            stack: vec![HashMap::new()],
            file_name,
            source,
        }
    }

    pub fn error(&self, message: &str, span: Span) {
        let error = Rich::<Token>::custom(span, message);
        print_error(error, self.source, self.file_name, "Check")
    }

    pub fn import(
        &mut self,
        use_: &SpannedQualifiedName,
        project: &'module Project,
        print_errors: bool,
    ) -> bool {
        let found = project.get_path_with_error(use_);
        match found {
            Ok(found) => {
                self.insert(use_.last().unwrap().0.to_string(), NamedExpr::Export(found));
                true
            }
            Err((name, span)) => {
                if print_errors {
                    self.error(&format!("Import not found '{name}'"), span);
                }
                false
            }
        }
    }
}

pub fn check_file(file: &FileState, project: &Project) {
    let source = &Source::from(file.text.clone());
    let mut state = CheckState::new(&file.filename, source);

    state.insert("String".to_string(), NamedExpr::Prim(PrimTy::String));
    state.insert("Bool".to_string(), NamedExpr::Prim(PrimTy::Bool));
    state.insert("Float".to_string(), NamedExpr::Prim(PrimTy::Float));
    state.insert("Int".to_string(), NamedExpr::Prim(PrimTy::Int));
    for (top, _) in &file.ast {
        if let Some(name) = top.get_name() {
            state.insert(name.to_string(), NamedExpr::Export(top.into()))
        }
    }
    for (item, _) in &file.ast {
        item.check(project, &mut state)
    }
}

#[cfg(test)]
mod tests {
    use crate::fs::project::Project;

    #[test]
    fn test_crud() {
        let project = Project::init_pwd();
        project.check();
    }
}
