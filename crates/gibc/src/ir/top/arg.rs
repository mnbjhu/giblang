use crate::{
    check::{
        scoped_state::Scoped as _,
        state::{CheckState, VarDecl},
        SemanticToken, TokenKind,
    },
    ir::{ty::TypeIR, ContainsOffset, IrNode, IrState},
    parser::top::arg::FunctionArg,
    util::Spanned,
};

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionArgIR<'db> {
    pub name: Spanned<String>,
    pub ty: Spanned<TypeIR<'db>>,
}

impl<'db> FunctionArg {
    pub fn check(&self, state: &mut CheckState<'db>) -> FunctionArgIR<'db> {
        let ty = (self.ty.0.check(state), self.ty.1);
        let var = VarDecl {
            name: self.name.0.clone(),
            ty: ty.0.ty.clone(),
            kind: TokenKind::Param,
            span: self.name.1,
        };
        state.insert_variable(&self.name.0, var);
        FunctionArgIR {
            name: self.name.clone(),
            ty,
        }
    }
}

impl<'db> IrNode<'db> for FunctionArgIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut IrState<'db>) -> &dyn IrNode {
        if self.ty.1.contains_offset(offset) {
            return self.ty.0.at_offset(offset, state);
        }
        self
    }

    fn tokens(&self, tokens: &mut Vec<SemanticToken>, state: &mut IrState<'db>) {
        tokens.push(SemanticToken {
            kind: TokenKind::Param,
            span: self.name.1,
        });
        self.ty.0.tokens(tokens, state);
    }

    fn debug_name(&self) -> &'static str {
        "FunctionArgIR"
    }
}
