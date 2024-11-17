use crate::{
    check::{build_state::BuildState, state::CheckState},
    ir::{
        expr::{ExprIR, ExprIRData},
        ContainsOffset, IrNode,
    },
    item::definitions::ident::IdentDef,
    parser::{expr::Expr, stmt::assign::Assign},
    run::bytecode::ByteCode,
    ty::Ty,
    util::Spanned,
};

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct AssignIR<'db> {
    pub refr: Spanned<ExprIR<'db>>,
    pub value: Spanned<ExprIR<'db>>,
}

impl<'db> Assign {
    pub fn check(&self, state: &mut CheckState<'db>) -> AssignIR<'db> {
        let refr = (self.refr.0.check(state), self.refr.1);
        let value = if let Ty::Unknown = &refr.0.ty {
            (self.value.0.check(state), self.value.1)
        } else {
            (
                self.value.0.expect(state, &refr.0.ty, self.value.1),
                self.value.1,
            )
        };
        if !matches!(self.refr.0, Expr::Field(_) | Expr::Ident(_)) {
            state.simple_error("Expected a ident or field", self.refr.1);
        };
        AssignIR { refr, value }
    }
}

impl<'db> IrNode<'db> for AssignIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut crate::ir::IrState<'db>) -> &dyn IrNode {
        if self.refr.1.contains_offset(offset) {
            return self.refr.0.at_offset(offset, state);
        }
        if self.value.1.contains_offset(offset) {
            return self.value.0.at_offset(offset, state);
        }
        self
    }

    fn tokens(
        &self,
        tokens: &mut Vec<crate::check::SemanticToken>,
        state: &mut crate::ir::IrState<'db>,
    ) {
        self.refr.0.tokens(tokens, state);
        self.value.0.tokens(tokens, state);
    }
}

impl<'db> AssignIR<'db> {
    pub fn build(&self, state: &mut BuildState<'db>) -> Vec<ByteCode> {
        match &self.refr.0.data {
            ExprIRData::Field(field) => {
                let mut code = self.refr.0.build(state);
                todo!()
            }
            ExprIRData::Ident(name) => match &name.last().unwrap().0 {
                IdentDef::Variable(var) => {
                    let id = state.get_var(&var.name).unwrap();
                    let mut code = self.value.0.build(state);
                    code.push(ByteCode::SetLocal(id));
                    code
                }
                _ => panic!("Don't think so?"),
            },
            _ => unreachable!(),
        }
    }
}
