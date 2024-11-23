use chumsky::container::Container;
use gvm::format::instr::ByteCode;

use crate::{
    check::{build_state::BuildState, state::CheckState},
    db::decl::{struct_::StructDecl, Decl, DeclKind},
    ir::{
        builder::ByteCodeNode,
        expr::{ExprIR, ExprIRData},
        ContainsOffset, IrNode,
    },
    item::definitions::ident::IdentDef,
    parser::{expr::Expr, stmt::assign::Assign},
    ty::Ty,
    util::Spanned,
};

#[derive(Debug, PartialEq, Clone)]
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

impl<'db> Decl<'db> {
    pub fn get_field_index(self, name: &str, state: &mut BuildState<'db>) -> u32 {
        let DeclKind::Struct {
            body: StructDecl::Fields(decl_fields),
            ..
        } = self.kind(state.db)
        else {
            panic!("Expected struct")
        };
        let mut index = decl_fields.iter().position(|(f, _)| f == name).unwrap();
        index = decl_fields.len() - index - 1;
        index as u32
    }
}

impl<'db> AssignIR<'db> {
    pub fn build(&self, state: &mut BuildState<'db>) -> ByteCodeNode {
        match &self.refr.0.data {
            ExprIRData::Field(field) => {
                let index = field.decl.unwrap().get_field_index(&field.name.0, state);
                let mut code = vec![field.struct_.0.build(state)];
                code.push(self.value.0.build(state));
                code.push(ByteCodeNode::Code(vec![ByteCode::SetIndex(index)]));
                ByteCodeNode::Block(code)
            }
            ExprIRData::Ident(name) => match &name.last().unwrap().0 {
                IdentDef::Variable(var) => {
                    let id = state.get_var(&var.name).unwrap();
                    let mut code = vec![self.value.0.build(state)];
                    code.push(ByteCodeNode::Code(vec![ByteCode::SetLocal(id)]));
                    ByteCodeNode::Block(code)
                }
                _ => panic!("Don't think so?"),
            },
            _ => unreachable!(),
        }
    }
}
