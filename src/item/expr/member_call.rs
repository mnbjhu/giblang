use std::{collections::HashMap, ops::ControlFlow};

use async_lsp::lsp_types::{CompletionItem, CompletionItemKind};

use crate::{
    check::{state::CheckState, SemanticToken, TokenKind},
    item::AstItem,
    parser::expr::member::MemberCall,
    ty::Ty,
};

use super::call::pretty_args;

impl AstItem for MemberCall {
    fn item_name(&self) -> &'static str {
        "member_call"
    }
    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        self.rec
            .0
            .pretty(allocator)
            .append(".")
            .append(&self.name.0)
            .append(pretty_args(&self.args, allocator))
    }

    fn tokens(&self, state: &mut CheckState, tokens: &mut Vec<SemanticToken>, _: &Ty<'_>) {
        todo!();
        // if let ControlFlow::Continue(rec) = self.rec.0.check(state, &mut (), self.rec.1, ()) {
        //     if rec.get_member_func(&self.name, state).is_some() {
        //         tokens.push(SemanticToken {
        //             span: self.name.1,
        //             kind: TokenKind::Func,
        //         });
        //     }
        // }
    }

    fn hover<'db>(
        &self,
        state: &mut CheckState<'db>,
        _: usize,
        type_vars: &HashMap<u32, Ty<'db>>,
        _: &Ty<'_>,
    ) -> Option<String> {
        // let ControlFlow::Continue(rec) = self.rec.0.check(state, &mut (), self.rec.1, ()) else {
        //     panic!("Unexpected ControlFlow::Break in Field::hover");
        // };
        // let func_ty = rec.get_member_func(&self.name, state)?;
        // Some(format!(
        //     "{}: {}",
        //     self.name.0,
        //     func_ty.get_name(state, Some(type_vars))
        // ))
        todo!()
    }

    fn completions(
        &self,
        state: &mut CheckState,
        _: usize,
        type_vars: &HashMap<u32, Ty>,
        _: &Ty,
    ) -> Vec<CompletionItem> {
        // let ControlFlow::Continue(rec) = self.rec.0.check(state, &mut (), self.rec.1, ()) else {
        //     panic!("Unexpected ControlFlow::Break in Field::completions");
        // };
        // let mut completions = Vec::new();
        // for (name, func_ty) in rec.member_funcs(state, self.name.1) {
        //     completions.push(CompletionItem {
        //         label: name.clone(),
        //         kind: Some(CompletionItemKind::METHOD),
        //         detail: Some(func_ty.get_name(state, Some(type_vars))),
        //         ..Default::default()
        //     });
        // }
        // completions
        todo!()
    }
}
