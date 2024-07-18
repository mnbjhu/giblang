use chumsky::{
    primitive::{choice, just},
    recursive::recursive,
    IterParser, Parser,
};

use crate::{
    lexer::token::punct,
    op,
    parser::expr::qualified_name::{qualified_name_parser, SpannedQualifiedName},
    util::Spanned,
    AstParser,
};

use super::optional_newline::optional_newline;

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Named(NamedType),
    Tuple(Vec<Spanned<Type>>),
    Sum(Vec<Spanned<Type>>),
    Function {
        receiver: Option<Box<Spanned<Type>>>,
        args: Vec<Spanned<Type>>,
        ret: Box<Spanned<Type>>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct NamedType {
    pub name: SpannedQualifiedName,
    pub args: Vec<Spanned<Type>>,
}

pub fn type_parser<'tokens, 'src: 'tokens>() -> AstParser!(Type) {
    let ident = qualified_name_parser();
    let arrow = just(punct('-')).then(just(punct('>'))).ignored();

    recursive(|ty| {
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
            .map(|args| args.unwrap_or_default());

        let named = ident
            .then(args)
            .map(|(name, args)| Type::Named(NamedType { name, args }));

        let tuple = ty
            .clone()
            .map_with(|t, s| (t, s.span()))
            .separated_by(just(punct(',')).padded_by(optional_newline()))
            .collect::<Vec<_>>()
            .delimited_by(
                just(punct('(')).then(optional_newline()),
                optional_newline().then(just(punct(')'))),
            );

        let bracketed = ty.clone().delimited_by(just(punct('(')), just(punct(')')));

        let atom = choice((named, bracketed, tuple.clone().map(Type::Tuple)));

        let sum = atom
            .clone()
            .map_with(|t, s| (t, s.span()))
            .separated_by(just(op!(+)).padded_by(optional_newline()))
            .collect::<Vec<_>>()
            .map(Type::Sum);

        let receiver = ty
            .clone()
            .map_with(|t, s| (t, s.span()))
            .then_ignore(just(punct('.')).padded_by(optional_newline()))
            .or_not();

        let function = receiver
            .then(tuple)
            .then_ignore(arrow)
            .then(ty.map_with(|t, e| (t, e.span())))
            .map(|((receiver, args), ret)| Type::Function {
                receiver: receiver.map(Box::new),
                args,
                ret: Box::new(ret),
            });

        choice((function, sum, atom))
    })
}

#[cfg(test)]
mod tests {
    use chumsky::{input::Input, Parser};

    use crate::{lexer::parser::lexer, parser::common::type_::Type, util::Span};

    use super::type_parser;

    #[test]
    fn test_named_type() {
        let input = "Foo";
        let tokens = lexer().parse(input).unwrap();
        let end = Span::splat(input.len());
        let input = tokens.spanned(end);
        let ty = type_parser().parse(input).unwrap();
        if let Type::Named(named) = ty {
            assert_eq!(named.name[0].0, "Foo");
            assert!(named.args.is_empty(), "expected no args");
        } else {
            panic!("expected named type");
        }
    }

    #[test]
    fn test_named_with_args() {
        let input = "Foo[Bar, Baz]";
        let tokens = lexer().parse(input).unwrap();
        let end = Span::splat(input.len());
        let input = tokens.spanned(end);
        let ty = type_parser().parse(input).unwrap();
        if let Type::Named(named) = ty {
            assert_eq!(named.name[0].0, "Foo");
            assert_eq!(named.args.len(), 2);
        } else {
            panic!("expected named type");
        }
    }

    #[test]
    fn named_with_nested_args() {
        let input = "Foo[Bar[Baz]]";
        let tokens = lexer().parse(input).unwrap();
        let end = Span::splat(input.len());
        let input = tokens.spanned(end);
        let ty = type_parser().parse(input).unwrap();
        if let Type::Named(ty) = ty {
            assert_eq!(ty.name[0].0, "Foo");
            assert_eq!(ty.args.len(), 1);
            let inner = &ty.args[0].0;
            if let Type::Named(inner) = inner {
                assert_eq!(inner.name[0].0, "Bar");
                assert_eq!(inner.args.len(), 1);
                if let Type::Named(inner) = &inner.args[0].0 {
                    assert_eq!(inner.name[0].0, "Baz");
                } else {
                    panic!("expected named type");
                }
            } else {
                panic!("expected named type");
            }
        }
    }
}
