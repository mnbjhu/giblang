use chumsky::{error::Rich, primitive::just, IterParser, Parser};

use crate::{
    fs::project::ImplData,
    kw,
    lexer::token::punct,
    parser::common::{
        generic_args::{generic_args_parser, GenericArgs},
        ident::spanned_ident_parser,
        optional_newline::optional_newline,
    },
    util::Spanned,
    AstParser,
};

use super::{
    enum_member::{enum_member_parser, EnumMember},
    impl_::Impl,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Enum {
    pub name: Spanned<String>,
    pub generics: Spanned<GenericArgs>,
    pub members: Vec<Spanned<EnumMember>>,
    pub impls: Vec<ImplData>,
}

pub fn enum_parser<'tokens, 'src: 'tokens>() -> AstParser!(Enum) {
    let members = enum_member_parser()
        .map_with(|t, s| (t, s.span()))
        .separated_by(just(punct(',')).padded_by(optional_newline()))
        .allow_trailing()
        .collect::<Vec<_>>()
        .delimited_by(
            just(punct('{')).then(optional_newline()),
            optional_newline().then(just(punct('}'))),
        )
        .validate(|mut v: Vec<Spanned<EnumMember>>, _, emitter| {
            let mut existing: Vec<String> = vec![];
            v.retain(|arg| {
                if existing.iter().any(|e| e == &arg.0.name.0) {
                    emitter.emit(Rich::custom(
                        arg.0.name.1,
                        format!("Duplicate definition of enum member '{}'", arg.0.name.0),
                    ));
                    false
                } else {
                    existing.push(arg.0.name.0.to_string());
                    true
                }
            });
            v
        });

    just(kw!(enum))
        .ignore_then(spanned_ident_parser())
        .then(generic_args_parser().map_with(|a, e| (a, e.span())))
        .then(members)
        .map(|((name, generics), members)| Enum {
            name,
            generics,
            members,
            impls: vec![],
        })
}
