use std::collections::HashMap;

use crate::{
    check::{build_state::BuildState, state::CheckState, Dir},
    item::AstItem,
    lexer::literal::Literal,
    run::bytecode::ByteCode,
};

impl AstItem for Literal {
    fn item_name(&self) -> &'static str {
        "literal"
    }
    fn hover<'db>(
        &self,
        _: &mut CheckState<'db>,
        _: usize,
        _: &HashMap<u32, crate::ty::Ty<'db>>,
        _: &crate::ty::Ty<'db>,
    ) -> Option<String> {
        let name = match self {
            Literal::Int(_) => "Int",
            Literal::Float(_) => "Float",
            Literal::String(_) => "String",
            Literal::Char(_) => "Char",
            Literal::Bool(_) => "Bool",
        };
        Some(name.to_string())
    }

    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        match self {
            Literal::Int(i) => allocator.text(i),
            Literal::Float(f) => allocator.text(f),
            Literal::String(s) => allocator.text(format!("\"{s}\"")),
            Literal::Char(c) => allocator.text(format!("'{c}'")),
            Literal::Bool(b) => {
                if *b {
                    allocator.text("true")
                } else {
                    allocator.text("false")
                }
            }
        }
    }

    fn build(&self, state: &mut CheckState<'_>, builder: &mut BuildState, dir: Dir) {
        if let Dir::Enter = dir {
            builder.add(ByteCode::Push(self.clone()));
        }
    }
}
