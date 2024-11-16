use std::collections::HashMap;

use top::TopIR;

use crate::{
    check::{build_state::BuildState, state::VarDecl, SemanticToken},
    db::{
        decl::{Decl, Project},
        input::Db,
        path::ModulePath,
    },
    run::state::FuncDef,
    ty::{Generic, Ty},
    util::{Span, Spanned},
};

pub mod common;
pub mod expr;
pub mod stmt;
pub mod top;
pub mod ty;

#[salsa::tracked]
pub struct FileIR<'db> {
    #[no_eq]
    #[return_ref]
    pub tops: Vec<Spanned<TopIR<'db>>>,
    pub imports: HashMap<String, Decl<'db>>,
    pub type_vars: HashMap<u32, Ty<'db>>,
}

pub trait IrNode<'db> {
    fn at_offset(&self, offset: usize, state: &mut IrState<'db>) -> &dyn IrNode;
    fn tokens(&self, tokens: &mut Vec<SemanticToken>, state: &mut IrState<'db>);
    #[allow(unused)]
    fn hover(&self, offset: usize, state: &mut IrState<'db>) -> Option<String> {
        None
    }
}

pub struct IrState<'db> {
    pub generics: Vec<HashMap<String, Generic<'db>>>,
    pub variables: Vec<HashMap<String, VarDecl<'db>>>,
    pub db: &'db dyn Db,
    pub project: Project<'db>,
    pub type_vars: HashMap<u32, Ty<'db>>,
}

impl<'db> IrState<'db> {
    pub fn new(db: &'db dyn Db, project: Project<'db>, type_vars: HashMap<u32, Ty<'db>>) -> Self {
        IrState {
            generics: vec![],
            variables: vec![],
            db,
            project,
            type_vars,
        }
    }

    pub fn enter_scope(
        &mut self,
        vars: HashMap<String, VarDecl<'db>>,
        generics: HashMap<String, Generic<'db>>,
    ) {
        self.variables.push(vars);
        self.generics.push(generics);
    }

    pub fn try_get_decl_path(&self, name: ModulePath<'db>) -> Option<Decl<'db>> {
        self.project.get_decl(self.db, name)
    }
}

impl<'db> FileIR<'db> {
    pub fn build(self, state: &mut BuildState<'db>) -> HashMap<u32, FuncDef> {
        self.tops(state.db)
            .iter()
            .filter_map(|(top, _)| top.build(state))
            .collect()
    }
}

impl<'db> IrNode<'db> for FileIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut IrState<'db>) -> &dyn IrNode {
        for (top, span) in self.tops(state.db) {
            if span.contains_offset(offset) {
                return top.at_offset(offset, state);
            }
        }
        self
    }

    fn tokens(&self, tokens: &mut Vec<SemanticToken>, state: &mut IrState<'db>) {
        for (top, _) in self.tops(state.db) {
            top.tokens(tokens, state);
        }
    }
}

pub trait ContainsOffset {
    fn contains_offset(&self, offset: usize) -> bool;
}

impl ContainsOffset for Span {
    fn contains_offset(&self, offset: usize) -> bool {
        self.start <= offset && offset <= self.end
    }
}
