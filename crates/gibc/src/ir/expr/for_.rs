use gvm::format::instr::ByteCode;
use salsa::plumbing::AsId;

use crate::{
    check::{build_state::BuildState, scoped_state::Scoped as _, state::CheckState},
    db::{
        decl::{Decl, DeclKind},
        path::ModulePath,
    },
    ir::{builder::ByteCodeNode, common::pattern::PatternIR, ContainsOffset, IrNode},
    item::definitions::ident::IdentDef,
    parser::expr::for_::For,
    ty::{Named, Ty},
    util::Spanned,
};

use super::{
    block::{check_block, CodeBlockIR},
    ExprIR, ExprIRData,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ForIR<'db> {
    pub pattern: Box<Spanned<PatternIR<'db>>>,
    pub expr: Box<Spanned<ExprIR<'db>>>,
    pub block: Spanned<CodeBlockIR<'db>>,
    pub iter_decl: Option<Decl<'db>>,
    pub next_decl: Option<Decl<'db>>,
}

impl<'db> For {
    pub fn check(&self, state: &mut CheckState<'db>) -> ExprIR<'db> {
        let expr = self.expr.0.check(state);

        let iter_decl = if let Some((IdentDef::Decl(decl), _)) = expr
            .ty
            .get_member_func(&("iter".to_string(), self.expr.1), state)
        {
            Some(decl)
        } else {
            None
        };

        let iter_ty = if let Some(into_iter_ty) = expr.ty.imply_named_sub_ty(
            ModulePath::new(state.db, vec!["std".to_string(), "IntoIter".to_string()]),
            state,
        ) {
            let Ty::Named(Named { name, args }) = into_iter_ty else {
                panic!("Expected a named type")
            };
            if name.name(state.db) == &["std", "IntoIter"] {
                args[0].clone()
            } else {
                if !matches!(expr.ty, Ty::Unknown) {
                    state.simple_error("Expected 'IntoIter'", self.expr.1);
                }
                Ty::Unknown
            }
        } else {
            state.simple_error(
                &format!(
                    "The type {} doesn't implement 'IntoIter'",
                    expr.ty.get_name(state)
                ),
                self.expr.1,
            );
            Ty::Unknown
        };

        let item_ty = if let Some(iter_ty) = iter_ty.imply_named_sub_ty(
            ModulePath::new(state.db, vec!["std".to_string(), "Iterator".to_string()]),
            state,
        ) {
            let Ty::Named(Named { name, args }) = iter_ty else {
                panic!("Expected a named type")
            };
            assert_eq!(name.name(state.db), &["std", "Iterator"]);
            args[0].clone()
        } else {
            if !matches!(iter_ty, Ty::Unknown) {
                state.simple_error(
                    &format!("Expected {} to be an 'Iterator'", iter_ty.get_name(state)),
                    self.expr.1,
                );
            }
            Ty::Unknown
        };

        let next_decl = if let Some((IdentDef::Decl(decl), _)) =
            iter_ty.get_member_func(&("next".to_string(), self.expr.1), state)
        {
            Some(decl)
        } else {
            None
        };

        let pattern = if matches!(item_ty, Ty::Unknown) {
            self.pattern.0.check(state)
        } else {
            self.pattern.0.expect(state, &item_ty)
        };
        let ExprIR {
            data: ExprIRData::CodeBlock(block),
            ..
        } = check_block(&self.block.0, state)
        else {
            panic!("Expected block")
        };
        let block = (block, self.block.1);
        ExprIR {
            data: ExprIRData::For(ForIR {
                pattern: Box::new((pattern, self.pattern.1)),
                expr: Box::new((expr, self.expr.1)),
                block,
                iter_decl,
                next_decl,
            }),
            ty: Ty::unit(),
            order: state.inc_order(),
        }
    }
}

impl<'db> IrNode<'db> for ForIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut crate::ir::IrState<'db>) -> &dyn IrNode {
        if self.pattern.1.contains_offset(offset) {
            return self.pattern.0.at_offset(offset, state);
        }
        if self.expr.1.contains_offset(offset) {
            return self.expr.0.at_offset(offset, state);
        }
        if self.block.1.contains_offset(offset) {
            return self.block.0.at_offset(offset, state);
        }
        self
    }

    fn tokens(
        &self,
        tokens: &mut Vec<crate::check::SemanticToken>,
        state: &mut crate::ir::IrState<'db>,
    ) {
        self.pattern.0.tokens(tokens, state);
        self.expr.0.tokens(tokens, state);
        self.block.0.tokens(tokens, state);
    }

    fn debug_name(&self) -> &'static str {
        "ForIR"
    }
}

impl<'db> ForIR<'db> {
    pub fn build(&self, state: &mut BuildState<'db>) -> ByteCodeNode {
        let expr = self.expr.0.build(state);
        let iter_id = self.iter_decl.unwrap().as_id().as_u32();
        let next_id = self.next_decl.unwrap().as_id().as_u32();
        let some_id = state
            .project
            .get_decl(
                state.db,
                ModulePath::new(
                    state.db,
                    vec!["std".to_string(), "Option".to_string(), "Some".to_string()],
                ),
            )
            .unwrap();
        let iter = if self.expr.0.ty.is_dyn(state.db, state.project) {
            ByteCode::DynCall(iter_id)
        } else {
            ByteCode::Call(iter_id)
        };
        let next = if matches!(
            self.iter_decl.unwrap().kind(state.db),
            DeclKind::Trait { .. }
        ) {
            ByteCode::DynCall(iter_id)
        } else {
            ByteCode::Call(next_id)
        };
        let create_iter = vec![expr, ByteCodeNode::Code(vec![iter])];

        let cond = ByteCodeNode::Block(vec![
            ByteCodeNode::Code(vec![
                ByteCode::Copy,
                next,
                ByteCode::Copy,
                ByteCode::Match(some_id.as_id().as_u32()),
            ]),
            ByteCodeNode::MaybeBreak,
        ]);

        let then = ByteCodeNode::Block(vec![
            ByteCodeNode::Code(vec![ByteCode::Index(0)]),
            self.pattern.0.build(state),
            self.block.0.build(state),
            ByteCodeNode::Continue,
        ]);
        ByteCodeNode::Block(vec![
            ByteCodeNode::Block(create_iter),
            ByteCodeNode::While(Box::new(cond), Box::new(then)),
        ])
    }
}
