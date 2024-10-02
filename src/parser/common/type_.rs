use chumsky::{
    primitive::{choice, just},
    recursive::recursive,
    IterParser, Parser,
};
use salsa::Update;

use crate::{
    lexer::token::punct,
    op,
    parser::expr::qualified_name::{qualified_name_parser, SpannedQualifiedName},
    util::{Span, Spanned},
    AstParser,
};

use super::optional_newline::optional_newline;

#[derive(Clone, Debug, PartialEq, Update, Eq)]
pub enum Type {
    Wildcard(Span),
    Named(NamedType),
    Tuple(Vec<Spanned<Type>>),
    Sum(Vec<Spanned<Type>>),
    Function {
        receiver: Option<Box<Spanned<Type>>>,
        args: Vec<Spanned<Type>>,
        ret: Box<Spanned<Type>>,
    },
}

#[derive(Clone, Debug, PartialEq, Update, Eq)]
pub struct NamedType {
    pub name: SpannedQualifiedName,
    pub args: Vec<Spanned<Type>>,
}

pub fn type_parser<'tokens, 'src: 'tokens>() -> AstParser!(Type) {
    let arrow = just(punct('-')).then(just(punct('>'))).ignored();
    let widlcard = just(op!(_)).map_with(|_, e| e.span()).map(Type::Wildcard);
    recursive(|ty| {
        let named = named_parser(ty.clone());

        let tuple = ty
            .clone()
            .map_with(|t, s| (t, s.span()))
            .separated_by(just(punct(',')).padded_by(optional_newline()))
            .allow_trailing()
            .collect::<Vec<_>>()
            .delimited_by(
                just(punct('(')).then(optional_newline()),
                optional_newline().then(just(punct(')'))),
            );

        let bracketed = ty.clone().delimited_by(just(punct('(')), just(punct(')')));

        let atom = choice((
            bracketed,
            tuple.clone().map(Type::Tuple),
            named.map(Type::Named),
        ));

        let sum = atom
            .clone()
            .map_with(|t, s| (t, s.span()))
            .separated_by(just(op!(+)).padded_by(optional_newline()))
            .at_least(2)
            .collect::<Vec<_>>()
            .map(Type::Sum);

        let receiver = atom
            .clone()
            .map_with(|t, s| (t, s.span()))
            .then_ignore(just(punct('.')).padded_by(optional_newline()))
            .or_not();

        let function = receiver
            .then(tuple)
            .then_ignore(arrow)
            .then(atom.clone().map_with(|t, e| (t, e.span())))
            .map(|((receiver, args), ret)| Type::Function {
                receiver: receiver.map(Box::new),
                args,
                ret: Box::new(ret),
            });

        choice((widlcard, function, sum, atom))
    })
}

pub fn named_parser<'tokens, 'src: 'tokens>(ty: AstParser!(Type)) -> AstParser!(NamedType) {
    let ident = qualified_name_parser();
    let args = ty
        .clone()
        .map_with(|t, s| (t, s.span()))
        .separated_by(just(punct(',')).padded_by(optional_newline()))
        .allow_trailing()
        .collect::<Vec<_>>()
        .delimited_by(
            just(punct('[')).then(optional_newline()),
            optional_newline().then(just(punct(']'))),
        )
        .or_not()
        .map(Option::unwrap_or_default);
    ident
        .then(args)
        .map(|(name, args)| NamedType { name, args })
}
