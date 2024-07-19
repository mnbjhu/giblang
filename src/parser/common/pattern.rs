use chumsky::{primitive::just, recursive::recursive, IterParser, Parser};

use crate::{
    lexer::token::punct,
    parser::expr::qualified_name::{qualified_name_parser, SpannedQualifiedName},
    util::Spanned,
    AstParser,
};

use super::{
    ident::{ident_parser, spanned_ident_parser},
    optional_newline::optional_newline,
};

#[derive(Clone, PartialEq, Debug)]
pub enum Pattern {
    Name(String),
    Struct {
        name: SpannedQualifiedName,
        fields: Vec<Spanned<StructFieldPattern>>,
    },
    UnitStruct(SpannedQualifiedName),
    TupleStruct {
        name: SpannedQualifiedName,
        fields: Vec<Spanned<Pattern>>,
    },
}

impl Pattern {
    pub fn name(&self) -> &SpannedQualifiedName {
        match self {
            Pattern::Struct { name, .. } => name,
            Pattern::TupleStruct { name, .. } => name,
            Pattern::UnitStruct(name) => name,
            Pattern::Name(_) => panic!("Name pattern has no name"),
        }
    }
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
    let sep = just(punct(':')).then(just(punct(':')));
    let unit = spanned_ident_parser()
        .separated_by(sep)
        .at_least(2)
        .collect()
        .map(Pattern::UnitStruct);

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

        let tuple_struct = qualified_name_parser()
            .then(tuple)
            .map(|(name, fields)| Pattern::TupleStruct { name, fields });

        let struct_ = struct_field_pattern_parser(pat)
            .map_with(|p, e| (p, e.span()))
            .separated_by(just(punct(',')).padded_by(optional_newline()))
            .collect()
            .delimited_by(
                just(punct('{')).then(optional_newline()),
                optional_newline().then(just(punct('}'))),
            );

        let struct_ = qualified_name_parser()
            .then(struct_)
            .map(|(name, fields)| Pattern::Struct { name, fields });

        tuple_struct.or(struct_).or(unit).or(name)
    })
}
