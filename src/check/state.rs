use std::collections::HashMap;

use ariadne::Source;
use chumsky::error::Rich;

use crate::{
    check::{ty::PrimTy, NamedExpr},
    cli::build::print_error,
    fs::{project::Project, tree_node::FileState, util::path_from_filename},
    lexer::token::Token,
    parser::expr::qualified_name::SpannedQualifiedName,
    util::{Span, Spanned},
};

pub struct CheckState<'module> {
    stack: Vec<HashMap<String, NamedExpr<'module>>>,
    file_name: &'module str,
    source: Source,
}

impl<'module> CheckState<'module> {
    pub fn from_file(file: &'module FileState) -> CheckState<'module> {
        let source = Source::from(file.text.clone());
        let mut state = CheckState::new(&file.filename, source);
        state.insert("String".to_string(), NamedExpr::Prim(PrimTy::String));
        state.insert("Bool".to_string(), NamedExpr::Prim(PrimTy::Bool));
        state.insert("Float".to_string(), NamedExpr::Prim(PrimTy::Float));
        state.insert("Int".to_string(), NamedExpr::Prim(PrimTy::Int));
        state.insert("Char".to_string(), NamedExpr::Prim(PrimTy::Char));
        for (top, _) in &file.ast {
            if let Some(name) = top.get_name() {
                let mut path = path_from_filename(&file.filename);
                path.push(name.to_string());
                state.insert(name.to_string(), NamedExpr::Imported(top.into(), path))
            }
        }
        state
    }

    pub fn enter_scope(&mut self) {
        self.stack.push(HashMap::new())
    }

    pub fn exit_scope(&mut self) {
        self.stack.pop();
    }

    pub fn get_name(&self, name: &str) -> NamedExpr<'module> {
        for scope in self.stack.iter().rev() {
            if let Some(found) = scope.get(name) {
                return found.clone();
            }
        }
        NamedExpr::Unknown
    }

    pub fn from(file: &'module FileState) -> Self {
        Self {
            stack: vec![HashMap::new()],
            file_name: &file.filename,
            source: Source::from(file.text.clone()),
        }
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
                    Ok(found) => {
                        let path = path.iter().map(|(name, _)| name).cloned().collect();
                        NamedExpr::Imported(found, path)
                    }
                    Err((name, span)) => {
                        if print_errors {
                            self.error(&format!("Import not found '{name}'"), span);
                        }
                        NamedExpr::Unknown
                    }
                }
            }
            NamedExpr::Imported(e, p) => {
                let found = e.get_path_with_error(&path[1..path.len()]);
                match found {
                    Ok(found) => {
                        let mut new = p.clone();
                        new.extend(path[1..path.len()].iter().map(|(name, _)| name).cloned());
                        NamedExpr::Imported(found, new)
                    }
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

    pub fn new(file_name: &'module str, source: Source) -> Self {
        Self {
            stack: vec![HashMap::new()],
            file_name,
            source,
        }
    }

    pub fn error(&self, message: &str, span: Span) {
        let error = Rich::<Token>::custom(span, message);
        print_error(error, self.source.clone(), self.file_name, "Check")
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
                let path = use_.iter().map(|(name, _)| name).cloned().collect();
                self.insert(
                    use_.last().unwrap().0.to_string(),
                    NamedExpr::Imported(found, path),
                );
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
