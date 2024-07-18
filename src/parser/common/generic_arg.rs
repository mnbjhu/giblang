use chumsky::{primitive::just, select, Parser};

use crate::{
    lexer::token::{punct, Token},
    util::Spanned,
    AstParser,
};

use super::{
    optional_newline::optional_newline,
    type_::{type_parser, Type},
    variance::{variance_parser, Variance},
};

#[derive(Clone, Debug, PartialEq)]
pub struct GenericArg {
    pub variance: Variance,
    pub name: Spanned<String>,
    pub super_: Option<Spanned<Type>>,
}

impl Default for GenericArg {
    fn default() -> Self {
        GenericArg {
            variance: Variance::Invariant,
            name: ("".to_string(), (0..0).into()),
            super_: None,
        }
    }
}

pub fn generic_arg_parser<'tokens, 'src: 'tokens>() -> AstParser!(GenericArg) {
    let super_ = just(punct(':'))
        .padded_by(optional_newline())
        .ignore_then(type_parser().map_with(|t, s| (t, s.span())))
        .or_not();

    let name = select! {
        Token::Ident(s) => s,
    }
    .map_with(|i, e| (i, e.span()));

    variance_parser()
        .then(name)
        .then(super_)
        .map(|((variance, name), super_)| GenericArg {
            variance,
            name,
            super_,
        })
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_parse_eq,
        parser::common::{
            generic_arg::{generic_arg_parser, GenericArg},
            type_::{NamedType, Type},
            variance::Variance,
        },
    };

    #[test]
    fn test_ident() {
        assert_parse_eq!(
            generic_arg_parser(),
            "T",
            GenericArg {
                variance: Variance::Invariant,
                name: ("T".to_string(), (0..1).into()),
                super_: None,
            }
        );
    }

    #[test]
    fn test_variance() {
        assert_parse_eq!(
            generic_arg_parser(),
            "out T",
            GenericArg {
                variance: Variance::Covariant,
                name: ("T".to_string(), (4..5).into()),
                super_: None,
            }
        );
        assert_parse_eq!(
            generic_arg_parser(),
            "in T",
            GenericArg {
                variance: Variance::Contravariant,
                name: ("T".to_string(), (3..4).into()),
                super_: None,
            }
        );
    }

    #[test]
    fn test_super() {
        let super_type = Type::Named(NamedType {
            name: vec![("Foo".to_string(), (3..6).into())],
            args: vec![],
        });

        assert_parse_eq!(
            generic_arg_parser(),
            "T: Foo",
            GenericArg {
                variance: Variance::Invariant,
                name: ("T".to_string(), (0..1).into()),
                super_: Some((super_type, (3..6).into())),
            }
        );
    }
}
