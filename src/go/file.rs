use std::collections::HashMap;

use crate::item::AstItem;

use super::func::GoFunc;

#[derive(Debug)]
pub struct GoFile {
    pub imports: HashMap<String, String>,
    pub funcs: HashMap<u32, GoFunc>,
}

impl AstItem for GoFile {
    fn item_name(&self) -> &'static str {
        "GoFile"
    }

    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        let block = allocator
            .intersperse(
                self.imports
                    .iter()
                    .map(|(name, loc)| pretty_import(name, loc, allocator)),
                allocator.hardline(),
            )
            .append(allocator.hardline())
            .nest(4);
        let imports = allocator
            .text("import")
            .append(allocator.space())
            .append("(")
            .append(allocator.hardline())
            .append(block)
            .append(")");
        let funcs = allocator.intersperse(
            self.funcs.values().map(|func| func.pretty(allocator)),
            allocator.hardline(),
        );
        imports.append(allocator.hardline()).append(funcs)
    }
}

fn pretty_import<'b, D, A>(
    name: &'b str,
    loc: &'b str,
    allocator: &'b D,
) -> pretty::DocBuilder<'b, D, A>
where
    D: pretty::DocAllocator<'b, A>,
    D::Doc: Clone,
    A: Clone,
{
    allocator
        .text(name)
        .append(allocator.space())
        .append("\"")
        .append(loc)
        .append("\"")
}
