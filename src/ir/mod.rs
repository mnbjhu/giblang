use std::collections::HashMap;

use top::TopIR;

use crate::{
    check::{state::VarDecl, SemanticToken},
    db::{input::Db, path::ModulePath},
    item::common::type_::ContainsOffset as _,
    ty::{Generic, Ty},
    util::Spanned,
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
    pub imports: HashMap<String, ModulePath<'db>>,
    pub type_vars: HashMap<u32, Ty<'db>>,
}

pub trait IrNode<'db> {
    fn at_offset(&self, offset: usize, state: &mut IrState<'db>) -> &dyn IrNode;
    fn tokens(&self, tokens: &mut Vec<SemanticToken>, state: &mut IrState<'db>);
    fn hover(&self, offset: usize, state: &mut IrState<'db>) -> Option<String> {
        None
    }
}

pub struct IrState<'db> {
    pub generics: Vec<HashMap<String, Generic<'db>>>,
    pub variables: Vec<HashMap<String, VarDecl<'db>>>,
    pub db: &'db dyn Db,
}

impl<'db> IrState<'db> {
    pub fn new(db: &'db dyn Db) -> Self {
        IrState {
            generics: vec![],
            variables: vec![],
            db,
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

    pub fn exit_scope(&mut self) {
        self.variables.pop().unwrap();
        self.generics.pop().unwrap();
    }

    pub fn get_generic(&self, name: &str) -> Option<&Generic<'db>> {
        for generics in self.generics.iter().rev() {
            if let Some(g) = generics.get(name) {
                return Some(g);
            }
        }
        None
    }

    pub fn get_variable(&self, name: &str) -> Option<VarDecl<'db>> {
        for variables in self.variables.iter().rev() {
            if let Some(v) = variables.get(name) {
                return Some(v.clone());
            }
        }
        None
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
