use crate::{
    check::state::CheckState,
    ir::{common::generic_args::GenericArgsIR, ty::TypeIR, ContainsOffset, IrNode},
    parser::top::impl_::Impl,
    run::state::FuncDef,
    util::Spanned,
};

use super::func::FuncIR;

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct ImplIR<'db> {
    pub generics: Spanned<GenericArgsIR<'db>>,
    pub trait_: Option<Spanned<TypeIR<'db>>>,
    pub for_: Spanned<TypeIR<'db>>,
    pub body: Vec<Spanned<FuncIR<'db>>>,
}

impl<'db> Impl {
    pub fn check(&self, state: &mut CheckState<'db>) -> ImplIR<'db> {
        let decl = state
            .project
            .impl_map(state.db)
            .values()
            .flatten()
            .find(|decl| decl.file(state.db) == state.file_data && decl.id(state.db) == self.id)
            .copied()
            .unwrap();
        let generics = (self.generics.0.check(state), self.generics.1);
        let for_ = (self.for_.0.check(state), self.for_.1);
        let trait_ = self
            .trait_
            .as_ref()
            .map(|(trait_, span)| (trait_.check(state), *span));
        state.add_self_ty(&for_.0.ty, self.for_.1);
        // TODO: Re-implement trait-func checking
        let body = self
            .body
            .iter()
            .enumerate()
            .map(|(index, (func, span))| {
                state.enter_scope();
                state.decl_stack.push(decl.functions(state.db)[index]);
                let ir = func.check(state, false);
                let _ = state.exit_scope();
                state.exit_decl();
                (ir, *span)
            })
            .collect();
        ImplIR {
            generics,
            trait_,
            for_,
            body,
        }
    }
}

impl<'db> IrNode<'db> for ImplIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut crate::ir::IrState<'db>) -> &dyn IrNode {
        if self.generics.1.contains_offset(offset) {
            return self.generics.0.at_offset(offset, state);
        }
        if self.for_.1.contains_offset(offset) {
            return self.for_.0.at_offset(offset, state);
        }
        if let Some((trait_, span)) = &self.trait_ {
            if span.contains_offset(offset) {
                return trait_.at_offset(offset, state);
            }
        }
        for (func, span) in &self.body {
            if span.contains_offset(offset) {
                return func.at_offset(offset, state);
            }
        }
        self
    }

    fn tokens(
        &self,
        tokens: &mut Vec<crate::check::SemanticToken>,
        state: &mut crate::ir::IrState<'db>,
    ) {
        self.generics.0.tokens(tokens, state);
        self.for_.0.tokens(tokens, state);
        if let Some((trait_, _)) = &self.trait_ {
            trait_.tokens(tokens, state);
        }
        for func in &self.body {
            func.0.tokens(tokens, state);
        }
    }
}

impl<'db> ImplIR<'db> {
    pub fn build(&self, state: &mut crate::ir::BuildState<'db>) -> Vec<(u32, FuncDef)> {
        let mut funcs = vec![];
        for func in &self.body {
            funcs.push(func.0.build(state));
        }
        funcs
    }
}
