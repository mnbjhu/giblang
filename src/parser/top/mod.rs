use chumsky::{primitive::choice, Parser};

use crate::{parser::stmt::stmt_parser, AstParser};

use self::{func::Func, struct_::Struct, trait_::Trait};

use super::expr::qualified_name::SpannedQualifiedName;

pub mod arg;
pub mod func;
pub mod impl_;
pub mod struct_;
pub mod struct_field;
pub mod trait_;
pub mod use_;

#[derive(Debug, PartialEq, Clone)]
pub enum Top {
    Func(func::Func),
    Struct(struct_::Struct),
    Trait(trait_::Trait),
    Impl(impl_::Impl),
    Use(SpannedQualifiedName),
}

pub fn top_parser<'tokens, 'src: 'tokens>() -> AstParser!(Top) {
    choice((
        func::func_parser(stmt_parser()).map(Top::Func),
        struct_::struct_parser().map(Top::Struct),
        trait_::trait_parser(stmt_parser()).map(Top::Trait),
        impl_::impl_parser(stmt_parser()).map(Top::Impl),
        use_::use_parser().map(Top::Use),
    ))
}

impl Top {
    pub fn name(&self) -> &str {
        match &self {
            Top::Func(Func { name, .. }) => &name.0,
            Top::Trait(Trait { name, .. }) => &name.0,
            Top::Struct(Struct { name, .. }) => &name.0,
            Top::Use(_) => unimplemented!("Use statement doesn't have a name"),
            Top::Impl(_) => unimplemented!("Impl statement doesn't have a name"),
        }
    }
}
