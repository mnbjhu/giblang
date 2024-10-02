use chumsky::{error::Rich, primitive::just, IterParser, Parser};

use crate::{
    lexer::token::punct,
    parser::common::{
        ident::spanned_ident_parser,
        optional_newline::optional_newline,
        type_::{type_parser, Type},
    },
    util::Spanned,
    AstParser,
};

#[derive(Clone, PartialEq, Debug, Eq)]
pub struct FunctionArg {
    pub name: Spanned<String>,
    pub ty: Spanned<Type>,
}

pub fn function_arg_parser<'tokens, 'src: 'tokens>() -> AstParser!(FunctionArg) {
    let name = spanned_ident_parser();
    let ty = type_parser().map_with(|t, e| (t, e.span()));

    name.then(just(punct(':')))
        .then(ty)
        .map(|((name, _), ty)| FunctionArg { name, ty })
}

type FunctionArgs = Vec<Spanned<FunctionArg>>;

pub fn function_args_parser<'tokens, 'src: 'tokens>() -> AstParser!(FunctionArgs) {
    function_arg_parser()
        .map_with(|a, e| (a, e.span()))
        .separated_by(just(punct(',')).padded_by(optional_newline()))
        .allow_trailing()
        .collect()
        .validate(|mut v: Vec<Spanned<FunctionArg>>, _, emitter| {
            let mut existing: Vec<String> = vec![];
            v.retain(|arg| {
                if existing.iter().any(|e| e == &arg.0.name.0) {
                    emitter.emit(Rich::custom(
                        arg.0.name.1,
                        format!(
                            "Duplicate definition of function argument '{}'",
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
        .delimited_by(
            just(punct('(')).then(optional_newline()),
            optional_newline().then(just(punct(')'))),
        )
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_parse_eq,
        parser::{
            common::type_::{NamedType, Type},
            top::arg::FunctionArg,
        },
    };

    #[test]
    fn test_function_arg_parser() {
        assert_parse_eq!(
            super::function_arg_parser(),
            "foo: Bar",
            FunctionArg {
                name: ("foo".to_string(), (0..3).into()),
                ty: (
                    Type::Named(NamedType {
                        name: vec![("Bar".to_string(), (5..8).into())],
                        args: vec![],
                    }),
                    (5..8).into()
                ),
            }
        );
    }

    #[test]
    fn test_function_args_parser() {
        assert_parse_eq!(
            super::function_args_parser(),
            "(foo: Bar, baz: Baz)",
            vec![
                (
                    FunctionArg {
                        name: ("foo".to_string(), (1..4).into()),
                        ty: (
                            Type::Named(NamedType {
                                name: vec![("Bar".to_string(), (6..9).into())],
                                args: vec![],
                            }),
                            (6..9).into()
                        ),
                    },
                    (1..9).into()
                ),
                (
                    FunctionArg {
                        name: ("baz".to_string(), (11..14).into()),
                        ty: (
                            Type::Named(NamedType {
                                name: vec![("Baz".to_string(), (16..19).into())],
                                args: vec![],
                            }),
                            (16..19).into()
                        ),
                    },
                    (11..19).into()
                ),
            ]
        );
    }
}
