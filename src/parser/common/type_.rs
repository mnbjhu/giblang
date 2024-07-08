use chumsky::{primitive::just, recursive::recursive, select, IterParser, Parser};

use crate::{
    lexer::token::{punct, Token},
    util::Spanned,
    AstParser,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Type {
    pub name: Spanned<String>,
    pub args: Vec<Spanned<Type>>,
}

pub fn type_parser<'tokens, 'src: 'tokens>() -> AstParser!(Type) {
    let ident = select! {
        Token::Ident(s) => s,
    }
    .map_with(|i, e| (i, e.span()));

    recursive(|ty| {
        let args = ty
            .map_with(|t, s| (t, s.span()))
            .separated_by(just(punct(',')))
            .allow_trailing()
            .collect()
            .delimited_by(just(punct('[')), just(punct(']')))
            .or_not()
            .map(|args| args.unwrap_or_default());

        ident.then(args).map(|(name, args)| Type { name, args })
    })
}

#[cfg(test)]
mod tests {
    use chumsky::{input::Input, Parser};

    use crate::{lexer::parser::lexer, util::Span};

    use super::type_parser;

    #[test]
    fn test_named_type() {
        let input = "Foo";
        let tokens = lexer().parse(input).unwrap();
        let end = Span::splat(input.len());
        let input = tokens.spanned(end);
        let ty = type_parser().parse(input).unwrap();
        assert_eq!(ty.name.0, "Foo");
        assert!(ty.args.is_empty(), "expected no args");
    }

    #[test]
    fn test_named_with_args() {
        let input = "Foo[Bar, Baz]";
        let tokens = lexer().parse(input).unwrap();
        let end = Span::splat(input.len());
        let input = tokens.spanned(end);
        let ty = type_parser().parse(input).unwrap();
        assert_eq!(ty.name.0, "Foo");
        assert_eq!(ty.args.len(), 2);
        assert_eq!(ty.args[0].0.name.0, "Bar");
        assert_eq!(ty.args[1].0.name.0, "Baz");
    }

    #[test]
    fn named_with_nested_args() {
        let input = "Foo[Bar[Baz]]";
        let tokens = lexer().parse(input).unwrap();
        let end = Span::splat(input.len());
        let input = tokens.spanned(end);
        let ty = type_parser().parse(input).unwrap();
        assert_eq!(ty.name.0, "Foo");
        assert_eq!(ty.args.len(), 1);
        let inner = &ty.args[0].0;
        assert_eq!(inner.name.0, "Bar");
        assert_eq!(inner.args.len(), 1);
        assert_eq!(inner.args[0].0.name.0, "Baz");
    }
}
