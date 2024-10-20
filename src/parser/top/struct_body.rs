use chumsky::{error::Rich, primitive::just, IterParser, Parser};

use crate::{
    lexer::token::punct,
    parser::common::{
        optional_newline::optional_newline,
        type_::{type_parser, Type},
    },
    util::Spanned,
    AstParser,
};

use super::struct_field::{struct_field_parser, StructField};

#[derive(Debug, PartialEq, Clone, Default, Eq)]
pub enum StructBody {
    #[default]
    None,
    Tuple(Vec<Spanned<Type>>),
    Fields(Vec<Spanned<StructField>>),
}

pub fn struct_body_parser<'tokens, 'src: 'tokens>() -> AstParser!(StructBody) {
    let fields = struct_field_parser()
        .map_with(|t, s| (t, s.span()))
        .separated_by(just(punct(',')).padded_by(optional_newline()))
        .allow_trailing()
        .collect::<Vec<_>>()
        .delimited_by(
            just(punct('{')).then(optional_newline()),
            optional_newline().then(just(punct('}'))),
        )
        .validate(|mut v: Vec<Spanned<StructField>>, _, emitter| {
            let mut existing: Vec<String> = vec![];
            v.retain(|arg| {
                if existing.iter().any(|e| e == &arg.0.name.0) {
                    emitter.emit(Rich::custom(
                        arg.0.name.1,
                        format!("Duplicate definition of struct field '{}'", arg.0.name.0),
                    ));
                    false
                } else {
                    existing.push(arg.0.name.0.to_string());
                    true
                }
            });
            v
        })
        .map(StructBody::Fields);

    let tuple = type_parser()
        .map_with(|t, e| (t, e.span()))
        .separated_by(just(punct(',')).padded_by(optional_newline()))
        .allow_trailing()
        .collect::<Vec<_>>()
        .delimited_by(
            just(punct('(')).then(optional_newline()),
            optional_newline().then(just(punct(')'))),
        )
        .map(StructBody::Tuple);

    fields
        .or(tuple)
        .or_not()
        .map(std::option::Option::unwrap_or_default)
}
