use chumsky::{primitive::choice, Parser};
use salsa::Update;

use crate::{parser::stmt::stmt_parser, util::{Span, Spanned}, AstParser};

use self::{enum_::Enum, func::Func, struct_::Struct, trait_::Trait};

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

#[derive(Debug, PartialEq, Clone, Update, Eq)]
pub enum Top {
    Func(Spanned<func::Func>),
    Struct(Spanned<struct_::Struct>),
    Enum(Spanned<enum_::Enum>),
    Trait(Spanned<trait_::Trait>),
    Impl(Spanned<impl_::Impl>),
    Use(SpannedQualifiedName),
}

pub fn top_parser<'tokens, 'src: 'tokens>() -> AstParser!(Top) {
    choice((
        func::func_parser(stmt_parser())
            .map_with(|func, e| Top::Func((func, e.span()))),
        struct_::struct_parser()
            .map_with(|s, e| Top::Struct((s, e.span()))),
        enum_::enum_parser()
            .map_with(|e, s| Top::Enum((e, s.span()))),
        trait_::trait_parser(stmt_parser())
            .map_with(|t, s| Top::Trait((t, s.span()))),
        impl_::impl_parser(stmt_parser())
            .map_with(|i, s| Top::Impl((i, s.span()))),
        use_::use_parser().map(Top::Use),
    ))
}

impl Top {
    #[must_use]
    pub fn get_name(&self) -> Option<&str> {
        match &self {
            Top::Func((Func { name, .. }, _))
            | Top::Trait((Trait { name, .. }, _))
            | Top::Struct((Struct { name, .. }, _))
            | Top::Enum((Enum { name, .. }, _)) => Some(&name.0),
            Top::Use(_) | Top::Impl(_) => None,
        }
    }

    #[must_use]
    pub fn name_span(&self) -> Span {
        match &self {
            Top::Func((Func { name, .. },_))
            | Top::Trait((Trait { name, .. }, _))
            | Top::Struct((Struct { name, .. }, _))
            | Top::Enum((Enum { name, .. }, _)) => name.1,
            Top::Impl(_) => unimplemented!("Impl statement doesn't have a name"),
            Top::Use(_) => unimplemented!("Use statement doesn't have a name"),
        }
    }
}
