use chumsky::{primitive::choice, Parser};

use crate::{parser::stmt::stmt_parser, util::Span, AstParser};

use self::{enum_::Enum, func::Func, impl_::Impl, struct_::Struct, trait_::Trait};

use super::expr::qualified_name::SpannedQualifiedName;

pub mod arg;
pub mod enum_;
pub mod enum_member;
pub mod func;
pub mod impl_;
pub mod struct_;
pub mod struct_body;
pub mod struct_field;
pub mod trait_;
pub mod use_;

#[derive(Debug, PartialEq, Clone)]
pub enum Top {
    Func(func::Func),
    Struct(struct_::Struct),
    Enum(enum_::Enum),
    Trait(trait_::Trait),
    Impl(impl_::Impl),
    Use(SpannedQualifiedName),
}

pub fn top_parser<'tokens, 'src: 'tokens>() -> AstParser!(Top) {
    choice((
        func::func_parser(stmt_parser()).map(Top::Func),
        struct_::struct_parser().map(Top::Struct),
        enum_::enum_parser().map(Top::Enum),
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
            Top::Enum(Enum { name, .. }) => &name.0,
            Top::Use(_) => unimplemented!("Use statement doesn't have a name"),
            Top::Impl(_) => unimplemented!("Impl statement doesn't have a name"),
        }
    }

    pub fn get_name(&self) -> Option<&str> {
        match &self {
            Top::Func(Func { name, .. }) => Some(&name.0),
            Top::Trait(Trait { name, .. }) => Some(&name.0),
            Top::Struct(Struct { name, .. }) => Some(&name.0),
            Top::Enum(Enum { name, .. }) => Some(&name.0),
            Top::Use(_) => None,
            Top::Impl(_) => None,
        }
    }

    pub fn name_span(&self) -> Span {
        match &self {
            Top::Func(Func { name, .. }) => name.1,
            Top::Trait(Trait { name, .. }) => name.1,
            Top::Struct(Struct { name, .. }) => name.1,
            Top::Enum(Enum { name, .. }) => name.1,
            Top::Impl(_) => unimplemented!("Impl statement doesn't have a name"),
            Top::Use(_) => unimplemented!("Use statement doesn't have a name"),
        }
    }

    pub fn get_id(&self) -> Option<u32> {
        match &self {
            Top::Func(Func { id, .. }) => Some(*id),
            Top::Trait(Trait { id, .. }) => Some(*id),
            Top::Struct(Struct { id, .. }) => Some(*id),
            Top::Enum(Enum { id, .. }) => Some(*id),
            Top::Use(_) => None,
            Top::Impl(Impl { id, .. }) => Some(*id),
        }
    }

    pub fn is_parent(&self) -> bool {
        match &self {
            Top::Func(_) => false,
            Top::Trait(_) => true,
            Top::Struct(_) => true,
            Top::Enum(_) => true,
            Top::Use(_) => false,
            Top::Impl(_) => true,
        }
    }
}
