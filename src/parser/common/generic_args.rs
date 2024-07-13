use chumsky::{error::Rich, primitive::just, IterParser, Parser};

use crate::{lexer::token::punct, util::Spanned, AstParser};

use super::{
    generic_arg::{generic_arg_parser, GenericArg},
    optional_newline::optional_newline,
};

#[derive(Clone, Debug, PartialEq)]
pub struct GenericArgs(pub Vec<Spanned<GenericArg>>);

pub fn generic_args_parser<'tokens, 'src: 'tokens>() -> AstParser!(GenericArgs) {
    generic_arg_parser()
        .map_with(|t, s| (t, s.span()))
        .separated_by(just(punct(',')).padded_by(optional_newline()))
        .allow_trailing()
        .collect::<Vec<_>>()
        .delimited_by(
            just(punct('[')).then(optional_newline()),
            optional_newline().then(just(punct(']'))),
        )
        .validate(|mut v: Vec<Spanned<GenericArg>>, _, emitter| {
            let mut existing: Vec<String> = vec![];
            v.retain(|arg| {
                if existing.iter().any(|e| e == &arg.0.name.0) {
                    emitter.emit(Rich::custom(
                        arg.0.name.1,
                        format!(
                            "Duplicate definition of generic argument '{}'",
                            arg.0.name.0
                        ),
                    ));
                    false
                } else {
                    existing.push(arg.0.name.0.to_string());
                    true
                }
            });
            v
        })
        .or_not()
        .map(|args| GenericArgs(args.unwrap_or_default()))
}

#[cfg(test)]
mod tests {
    use chumsky::input::Input;
    use chumsky::Parser;

    use crate::{
        assert_parse_eq,
        lexer::parser::lexer,
        parser::common::{generic_args::GenericArgs, variance::Variance},
        util::Span,
    };

    use super::generic_args_parser;

    #[test]
    fn test_generic_args() {
        let input = "[in T, U: Thing]";
        let tokens = lexer().parse(input).unwrap();
        let input = tokens.spanned(Span::splat(input.len()));
        let args = generic_args_parser().parse(input).unwrap().0;
        assert_eq!(args.len(), 2);
        assert_eq!(args[0].0.name.0, "T");
        assert_eq!(args[0].0.variance, Variance::Contravariant);
        assert_eq!(args[1].0.name.0, "U");
        assert_eq!(args[1].0.variance, Variance::Invariant);
        if let Some(super_) = args[1].0.super_.as_ref() {
            assert_eq!(super_.0.name[0].0, "Thing");
            assert_eq!(super_.0.args.len(), 0);
        } else {
            panic!("expected super");
        }
    }

    #[test]
    fn test_generic_args_empty() {
        assert_parse_eq!(generic_args_parser(), "[]", GenericArgs(vec![]));
    }
}
