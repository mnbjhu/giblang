use chumsky::{primitive::just, recursive::recursive, IterParser, Parser};

use crate::{lexer::token::punct, util::Spanned, AstParser};

use super::{
    ident::{ident_parser, spanned_ident_parser},
    optional_newline::optional_newline,
};

#[derive(Clone, PartialEq, Debug)]
pub enum Pattern {
    Name(String),
    Struct {
        name: Spanned<String>,
        fields: Vec<StructFieldPattern>,
    },
    TupleStruct {
        name: Spanned<String>,
        fields: Vec<Spanned<Pattern>>,
    },
}

#[derive(Clone, PartialEq, Debug)]
pub enum StructFieldPattern {
    Implied(String),
    Explicit {
        field: Spanned<String>,
        pattern: Spanned<Pattern>,
    },
}

pub fn struct_field_pattern_parser<'tokens, 'src: 'tokens>(
    pattern: AstParser!(Pattern),
) -> AstParser!(StructFieldPattern) {
    let implied = ident_parser().map(StructFieldPattern::Implied);

    let explicit = spanned_ident_parser()
        .then_ignore(just(punct(':')))
        .then(pattern.map_with(|p, e| (p, e.span())))
        .map(|(field, pattern)| StructFieldPattern::Explicit { field, pattern });

    explicit.or(implied)
}

pub fn pattern_parser<'tokens, 'src: 'tokens>() -> AstParser!(Pattern) {
    let name = ident_parser().map(Pattern::Name);
    recursive(|pat| {
        let tuple = pat
            .clone()
            .map_with(|p, e| (p, e.span()))
            .separated_by(just(punct(',')).padded_by(optional_newline()))
            .collect()
            .delimited_by(
                just(punct('(')).then(optional_newline()),
                optional_newline().then(just(punct(')'))),
            );

        let tuple_struct = spanned_ident_parser()
            .then(tuple)
            .map(|(name, fields)| Pattern::TupleStruct { name, fields });

        let struct_ = struct_field_pattern_parser(pat)
            .separated_by(just(punct(',')).padded_by(optional_newline()))
            .collect()
            .delimited_by(
                just(punct('{')).then(optional_newline()),
                optional_newline().then(just(punct('}'))),
            );

        let struct_ = spanned_ident_parser()
            .then(struct_)
            .map(|(name, fields)| Pattern::Struct { name, fields });

        tuple_struct.or(struct_).or(name)
    })
}
