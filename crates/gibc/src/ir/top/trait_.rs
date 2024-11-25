use gvm::format::func::FuncDef;

use crate::{
    check::{scoped_state::Scoped as _, state::CheckState, SemanticToken, TokenKind},
    ir::{common::generic_args::GenericArgsIR, ContainsOffset, IrNode},
    parser::top::trait_::Trait,
    ty::{Named, Ty},
    util::Spanned,
};

use super::func::FuncIR;

#[derive(Debug, PartialEq, Clone)]
pub struct TraitIR<'db> {
    pub name: Spanned<String>,
    pub generics: Spanned<GenericArgsIR<'db>>,
    pub body: Vec<Spanned<FuncIR<'db>>>,
}

impl<'db> Trait {
    pub fn check(&self, state: &mut CheckState<'db>) -> TraitIR<'db> {
        let generics = (self.generics.0.check(state), self.generics.1);
        let decl = state.current_decl();
        state.add_self_ty(
            &Ty::Named(Named {
                name: decl.path(state.db),
                args: generics.0 .0.iter().map(|gen| gen.0.get_ty()).collect(),
            }),
            self.name.1,
        );
        let body = self
            .body
            .iter()
            .map(|(func, span)| {
                state.enter_scope();
                state.enter_decl(&func.name.0);
                let mut ir = func.check(state, true);
                let scope = state.exit_scope();
                ir.scope = Some(scope);
                state.exit_decl();
                (ir, *span)
            })
            .collect();
        TraitIR {
            name: self.name.clone(),
            generics,
            body,
        }
    }
}

impl<'db> IrNode<'db> for TraitIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut crate::ir::IrState<'db>) -> &dyn IrNode {
        if self.generics.1.contains_offset(offset) {
            return self.generics.0.at_offset(offset, state);
        }
        for (func, span) in &self.body {
            if span.contains_offset(offset) {
                state.push_scope(func.scope.as_ref().unwrap().clone());
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
        tokens.push(SemanticToken {
            span: self.name.1,
            kind: TokenKind::Trait,
        });
        self.generics.0.tokens(tokens, state);
        for func in &self.body {
            func.0.tokens(tokens, state);
        }
    }

    fn debug_name(&self) -> &'static str {
        "TraitIR"
    }
}

impl<'db> TraitIR<'db> {
    pub fn build(&self, state: &mut crate::ir::BuildState<'db>) -> Vec<(u32, FuncDef)> {
        let mut funcs = vec![];
        for func in &self.body {
            funcs.push(func.0.build(state));
        }
        funcs
    }
}
