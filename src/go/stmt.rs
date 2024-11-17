use crate::item::AstItem;

use super::expr::GoExpr;

#[derive(Debug)]
pub enum GoStmt {
    VarDecl { names: Vec<String>, expr: GoExpr },
    Expr(GoExpr),
}

impl AstItem for GoStmt {
    fn item_name(&self) -> &'static str {
        "GoStmt"
    }

    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        let sep = allocator.text(",").append(allocator.space());

        match self {
            GoStmt::VarDecl { names, expr } => {
                let names = names.iter().map(|name| allocator.text(name));
                let names = allocator.intersperse(names, sep);
                names
                    .append(allocator.space())
                    .append(allocator.text(":="))
                    .append(allocator.space())
                    .append(expr.pretty(allocator))
            }
            GoStmt::Expr(expr) => expr.pretty(allocator),
        }
    }
}
