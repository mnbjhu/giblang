use chumsky::{error::Rich, primitive::just, span::Span as _, IterParser, Parser};

use crate::{
    lexer::token::punct,
    parser::common::ident::spanned_ident_parser,
    util::{Span, Spanned},
    AstParser,
};

pub type SpannedQualifiedName = Vec<Spanned<String>>;

#[must_use]
pub fn qualified_name_parser<'tokens, 'src: 'tokens>() -> AstParser!(SpannedQualifiedName) {
    let separator = just(punct(':')).then(just(punct(':')));
    spanned_ident_parser()
        .separated_by(separator.clone())
        .at_least(1)
        .collect::<Vec<_>>()
        .then(separator.or_not())
        .validate(|(mut v, sep), e, emitter| {
            if sep.is_some() {
                v.push((String::new(), Span::to_end(&e.span())));
                emitter.emit(Rich::custom(
                    Span::to_end(&e.span()),
                    "Expected more parts in qualified name".to_string(),
                ));
            }
            v
        })
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_parse_eq, assert_parse_eq_with_errors,
        parser::expr::qualified_name::qualified_name_parser,
    };

    #[test]
    fn test_ident() {
        assert_parse_eq!(
            qualified_name_parser(),
            "thing",
            vec![("thing".to_string(), (0..5).into())]
        );
    }

    #[test]
    fn test_ident_recover() {
        assert_parse_eq_with_errors!(
            qualified_name_parser(),
            "thing::",
            vec![
                ("thing".to_string(), (0..5).into()),
                (String::new(), (7..7).into())
            ]
        );
    }
}
