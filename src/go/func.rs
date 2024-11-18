use crate::{
    item::{
        common::generics::{brackets, comma_sep_braces},
        AstItem,
    },
    util::Spanned,
};

use super::{stmt::GoStmt, ty::GoType};

#[derive(Debug)]
pub struct GoFunc {
    pub id: u32,
    pub receiver: Option<GoType>,
    pub args: Vec<GoType>,
    pub ret: Option<GoType>,
    pub body: Vec<GoStmt>,
}

impl AstItem for GoFunc {
    fn item_name(&self) -> &'static str {
        todo!()
    }

    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        let receiver = if let Some(receiver) = &self.receiver {
            allocator
                .text("(")
                .append("self")
                .append(allocator.space())
                .append(receiver.pretty(allocator))
                .append(")")
                .append(allocator.space())
        } else {
            allocator.nil()
        };
        let ret = if let Some(ret) = &self.ret {
            allocator.space().append(ret.pretty(allocator))
        } else {
            allocator.nil()
        };
        allocator
            .text("func")
            .append(allocator.space())
            .append(receiver)
            .append(format!("F{}", self.id))
            .append(brackets(allocator, "(", ")", &self.args))
            .append(ret)
            .append(comma_sep_braces(allocator, &self.body))
    }
}

impl<T: AstItem> AstItem for Spanned<T> {
    fn item_name(&self) -> &'static str {
        self.0.item_name()
    }

    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        self.0.pretty(allocator)
    }
}
