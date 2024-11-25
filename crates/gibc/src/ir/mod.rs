use std::collections::HashMap;

use async_lsp::lsp_types::CompletionItem;
use gvm::format::ByteCodeFile;
use salsa::plumbing::AsId;
use top::TopIR;

use crate::{
    check::{
        build_state::BuildState,
        is_scoped::IsScoped,
        scoped_state::{Scope, Scoped, ScopedState},
        SemanticToken,
    },
    db::{
        decl::{Decl, Project},
        input::{Db, SourceFile},
        path::ModulePath,
    },
    ty::Ty,
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

    #[no_eq]
    #[return_ref]
    pub scope: Scope<'db>,
    pub type_vars: HashMap<u32, Ty<'db>>,
}

pub trait IrNode<'db> {
    fn debug_name(&self) -> &'static str;
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
    #[allow(unused)]
    fn completions(&self, offset: usize, state: &mut IrState<'db>) -> Vec<CompletionItem> {
        vec![]
    }
}

pub struct IrState<'db> {
    pub db: &'db dyn Db,
    pub project: Project<'db>,
    pub type_vars: HashMap<u32, Ty<'db>>,
    pub file: SourceFile,
    pub scope_state: ScopedState<'db>,
    pub kind: AstKind,
}

#[derive(PartialEq)]
pub enum AstKind {
    Expr,
    Pattern,
    Type,
    Stmt,
    Decl,
}

impl<'db> IsScoped<'db> for IrState<'db> {
    fn get_scope_state<'me>(&'me self) -> &'me ScopedState<'db> {
        &self.scope_state
    }

    fn get_scope_state_mut<'me>(&'me mut self) -> &'me mut ScopedState<'db> {
        &mut self.scope_state
    }

    fn get_type_var(&self, id: u32) -> Ty<'db> {
        self.type_vars.get(&id).cloned().unwrap_or(Ty::Unknown)
    }
}

impl<'db> IrState<'db> {
    pub fn new(
        db: &'db dyn Db,
        project: Project<'db>,
        type_vars: HashMap<u32, Ty<'db>>,
        file: SourceFile,
    ) -> Self {
        IrState {
            db,
            project,
            type_vars,
            file,
            scope_state: ScopedState::new(db, project, file),
            kind: AstKind::Decl,
        }
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
            funcs,
            tables,
            file_names,
        }
    }
}

impl<'db> IrNode<'db> for FileIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut IrState<'db>) -> &dyn IrNode {
        state.push_scope(self.scope(state.db).clone());
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

    fn debug_name(&self) -> &'static str {
        "FileIR"
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
