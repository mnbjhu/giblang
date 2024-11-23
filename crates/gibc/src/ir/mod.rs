use std::collections::HashMap;

use gvm::format::ByteCodeFile;
use salsa::plumbing::AsId;
use top::TopIR;

use crate::{
    check::{build_state::BuildState, state::VarDecl, SemanticToken},
    db::{
        decl::{Decl, Project},
        input::{Db, SourceFile},
        path::ModulePath,
    },
    ty::{Generic, Ty},
    util::{Span, Spanned},
};

pub mod builder;
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
    #[allow(unused)]
    fn goto(&self, offset: usize, state: &mut IrState<'db>) -> Option<(SourceFile, Span)> {
        None
    }
}

pub struct IrState<'db> {
    pub generics: Vec<HashMap<String, Generic<'db>>>,
    pub variables: Vec<HashMap<String, VarDecl<'db>>>,
    pub db: &'db dyn Db,
    pub project: Project<'db>,
    pub type_vars: HashMap<u32, Ty<'db>>,
    pub file: SourceFile,
}

impl<'db> IrState<'db> {
    pub fn new(
        db: &'db dyn Db,
        project: Project<'db>,
        type_vars: HashMap<u32, Ty<'db>>,
        file: SourceFile,
    ) -> Self {
        IrState {
            generics: vec![],
            variables: vec![],
            db,
            project,
            type_vars,
            file,
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

    pub fn get_var(&self, name: &str) -> Option<&VarDecl<'db>> {
        for scope in self.variables.iter().rev() {
            if let Some(var) = scope.get(name) {
                return Some(var);
            }
        }
        None
    }

    pub fn get_generic(&self, name: &str) -> Option<&Generic<'db>> {
        for scope in self.generics.iter().rev() {
            if let Some(generic) = scope.get(name) {
                return Some(generic);
            }
        }
        None
    }

    pub fn exit_scope(&mut self) {
        self.variables.pop();
        self.generics.pop();
    }

    pub fn try_get_decl_path(&self, name: ModulePath<'db>) -> Option<Decl<'db>> {
        self.project.get_decl(self.db, name)
    }
}

impl<'db> FileIR<'db> {
    pub fn build(self, state: &mut BuildState<'db>) -> ByteCodeFile {
        let funcs = self
            .tops(state.db)
            .iter()
            .flat_map(|(top, _)| top.build(state))
            .collect();
        let tables = state.vtables.clone();
        let file_names = state.db.files();
        let file_names = file_names
            .iter()
            .map(|f| {
                (
                    f.as_id().as_u32(),
                    f.path(state.db)
                        .to_string_lossy()
                        .strip_prefix(&state.db.root())
                        .unwrap()
                        .strip_prefix("/")
                        .unwrap()
                        .to_string(),
                )
            })
            .collect();
        ByteCodeFile {
            file_names,
            funcs,
            tables,
        }
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
