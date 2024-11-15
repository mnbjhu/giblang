use chumsky::container::Container;

use crate::{
    check::{state::CheckState, SemanticToken, TokenKind},
    ir::{common::generic_args::GenericArgsIR, IrNode},
    item::common::type_::ContainsOffset,
    parser::top::trait_::Trait,
    ty::{Named, Ty},
    util::Spanned,
};

use super::func::FuncIR;

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct TraitIR<'db> {
    pub name: Spanned<String>,
    pub generics: Spanned<GenericArgsIR<'db>>,
    pub body: Vec<Spanned<FuncIR<'db>>>,
}

impl<'db> Trait {
    pub fn check(&self, state: &mut CheckState<'db>) -> TraitIR<'db> {
        let generics = (self.generics.0.check(state), self.generics.1);
        let id = state.local_id(self.name.0.to_string());
        state.add_self_ty(
            &Ty::Named(Named {
                name: id,
                args: generics.0 .0.iter().map(|gen| gen.0.get_ty()).collect(),
            }),
            self.name.1,
        );
        let body = self
            .body
            .iter()
            .map(|(func, span)| {
                state.enter_scope();
                let ir = func.check(state, true);
                state.exit_scope();
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
}
