use chumsky::{
    primitive::{choice, just},
    recursive::recursive,
    select, IterParser, Parser,
};
use gvm::format::literal::Literal;

use crate::{
    lexer::token::{punct, Token},
    op,
    parser::expr::qualified_name::{qualified_name_parser, SpannedQualifiedName},
    util::{Span, Spanned},
    AstParser,
};

use super::{ident::spanned_ident_parser, optional_newline::optional_newline};

#[derive(Clone, PartialEq, Debug)]
pub enum Pattern {
    Name(Spanned<String>),
    Struct {
        name: SpannedQualifiedName,
        fields: Vec<Spanned<StructFieldPattern>>,
    },
    UnitStruct(SpannedQualifiedName),
    TupleStruct {
        name: SpannedQualifiedName,
        fields: Vec<Spanned<Pattern>>,
    },
    Exact(Spanned<Literal>),
    Wildcard(Span),
}

impl Pattern {
    #[must_use]
    pub fn name(&self) -> &SpannedQualifiedName {
        match self {
            Pattern::Struct { name, .. }
            | Pattern::TupleStruct { name, .. }
            | Pattern::UnitStruct(name) => name,
            Pattern::Name(_) => panic!("Name pattern has no name"),
            Pattern::Exact(_) => panic!("Exact pattern has no name"),
            Pattern::Wildcard(_) => panic!("Wildcard pattern has no name"),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum StructFieldPattern {
    Implied(Spanned<String>),
    Explicit {
        field: Spanned<String>,
        pattern: Spanned<Pattern>,
    },
}

pub fn struct_field_pattern_parser<'tokens, 'src: 'tokens>(
    pattern: AstParser!(Pattern),
) -> AstParser!(StructFieldPattern) {
    let implied = spanned_ident_parser().map(StructFieldPattern::Implied);

    let explicit = spanned_ident_parser()
        .then_ignore(just(punct(':')))
        .then(pattern.map_with(|p, e| (p, e.span())))
        .map(|(field, pattern)| StructFieldPattern::Explicit { field, pattern });

    explicit.or(implied)
}

pub fn pattern_parser<'tokens, 'src: 'tokens>() -> AstParser!(Pattern) {
    let name = spanned_ident_parser().map(Pattern::Name);
    let wildcard = just(op!(_)).map_with(|_, e| Pattern::Wildcard(e.span()));
    let exact = select! {
        Token::Literal(lit) => lit,
    }
    .map_with(|lit, e| (lit, e.span()))
    .map(Pattern::Exact);

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
            .allow_trailing()
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
            .allow_trailing()
            .collect()
            .delimited_by(
                just(punct('{')).then(optional_newline()),
                optional_newline().then(just(punct('}'))),
            );

        let struct_ = qualified_name_parser()
            .then(struct_)
            .map(|(name, fields)| Pattern::Struct { name, fields });

        choice((tuple_struct, struct_, unit, name, exact, wildcard))
    })
}
