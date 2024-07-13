use std::collections::HashMap;

use ariadne::Source;
use chumsky::error::Rich;

use crate::{
    cli::build::print_error,
    fs::{export::Export, project::Project, tree_node::FileState},
    lexer::token::Token,
    parser::{common::variance::Variance, File},
    util::Span,
};

use self::ty::Ty;

pub mod top;
pub mod ty;

pub struct CheckState<'module> {
    stack: Vec<HashMap<String, NamedExpr<'module>>>,
    file_name: &'module str,
    source: &'module Source,
}

#[derive(Clone)]
pub enum NamedExpr<'module> {
    Export(Export<'module>),
    Variable(Ty<'module>),
    GenericArg {
        super_: Ty<'module>,
        variance: Variance,
    },
}

impl<'module> CheckState<'module> {
    pub fn enter_scope(&mut self) {
        self.stack.push(HashMap::new())
    }

    pub fn exit_scope(&mut self) {
        self.stack.pop();
    }

    pub fn get(&self, name: &str) -> Option<NamedExpr<'module>> {
        self.stack
            .iter()
            .rev()
            .find_map(|stack| stack.get(name))
            .cloned()
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
        print_error(error, self.source, "Check", self.file_name)
    }
}

pub fn check_file<'module>(file: &FileState, project: &'module Project) {
    let source = &Source::from(file.text.clone());
    let mut state = CheckState::new(&file.filename, &source);
    for (item, _) in &file.ast {
        item.check(project, &mut state)
    }
}
