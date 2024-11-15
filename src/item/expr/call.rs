use salsa::plumbing::AsId;

use crate::{
    check::{build_state::BuildState, state::CheckState, Dir},
    db::{decl::DeclKind, path::ModulePath},
    item::{common::generics::brackets, AstItem},
    parser::expr::{call::Call, Expr},
    run::bytecode::ByteCode,
    util::Spanned,
};

impl AstItem for Call {
    fn item_name(&self) -> &'static str {
        "call"
    }
    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        self.name
            .0
            .pretty(allocator)
            .append(pretty_args(&self.args, allocator))
    }

    fn build(&self, state: &mut CheckState<'_>, builder: &mut BuildState, dir: Dir) {
        if matches!(dir, Dir::Enter) {
            return;
        }
        if let Expr::Ident(name) = self.name.0.as_ref() {
            let decl = state.get_decl_with_error(name);
            if let Ok(decl) = decl {
                match decl.kind(state.db) {
                    DeclKind::Struct { generics, body } => todo!(),
                    DeclKind::Trait { generics, body } => todo!(),
                    DeclKind::Enum { generics, variants } => todo!(),
                    DeclKind::Member { body } => todo!(),
                    DeclKind::Function(_) => {
                        builder.add(ByteCode::Call(decl.path(state.db).as_id().as_u32()));
                    }
                    DeclKind::Module(_) => todo!(),
                }
            } else {
                todo!()
            }
        } else {
            todo!()
        }
    }
}

pub fn pretty_args<'b, D, A>(
    args: &'b [Spanned<Expr>],
    allocator: &'b D,
) -> pretty::DocBuilder<'b, D, A>
where
    D: pretty::DocAllocator<'b, A>,
    D::Doc: Clone,
    A: Clone,
{
    if args.len() == 1 {
        if let Expr::Lambda(l) = &args[0].0 {
            return allocator.space().append(l.pretty(allocator));
        }
    }
    if let Some((Expr::Lambda(l), _)) = args.last() {
        brackets(allocator, "(", ")", &args[..args.len() - 1])
            .append(allocator.space())
            .append(l.pretty(allocator))
    } else {
        brackets(allocator, "(", ")", args)
    }
}
