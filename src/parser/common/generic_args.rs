use chumsky::{error::Rich, primitive::just, IterParser, Parser};

use crate::{lexer::token::punct, util::Spanned, AstParser};

use super::{
    generic_arg::{generic_arg_parser, GenericArg},
    optional_newline::optional_newline,
};

#[derive(Clone, Debug, PartialEq, Eq)]
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
