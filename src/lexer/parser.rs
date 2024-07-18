use chumsky::{
    error::Rich,
    extra,
    primitive::{choice, end, just, none_of, one_of},
    text, IterParser, Parser,
};

use crate::util::Spanned;

use super::{keyword::Keyword, literal::Literal, token::Token};

pub fn lexer<'src>(
) -> impl Parser<'src, &'src str, Vec<Spanned<Token>>, extra::Err<Rich<'src, char>>> {
    let ident = text::ident().map(|ident| match ident {
        "fn" => Token::Keyword(Keyword::Fn),
        "let" => Token::Keyword(Keyword::Let),
        "struct" => Token::Keyword(Keyword::Struct),
        "enum" => Token::Keyword(Keyword::Enum),
        "use" => Token::Keyword(Keyword::Use),
        "in" => Token::Keyword(Keyword::In),
        "out" => Token::Keyword(Keyword::Out),
        "trait" => Token::Keyword(Keyword::Trait),
        "impl" => Token::Keyword(Keyword::Impl),
        "for" => Token::Keyword(Keyword::For),
        "match" => Token::Keyword(Keyword::Match),
        "if" => Token::Keyword(Keyword::If),
        "else" => Token::Keyword(Keyword::Else),
        "true" => Token::Literal(Literal::Bool(true)),
        "false" => Token::Literal(Literal::Bool(false)),
        _ => Token::Ident(ident.to_string()),
    });

    let string = none_of("\"")
        .repeated()
        .to_slice()
        .map(|s: &str| Token::Literal(Literal::String(s.to_string())))
        .delimited_by(just('"'), just('"'));

    let digits = text::digits(10).repeated().at_least(1);

    let float = digits
        .then(just('.'))
        .then(digits)
        .to_slice()
        .map(|s: &str| Token::Literal(Literal::Float(s.to_string())));

    let int = digits
        .to_slice()
        .map(|s: &str| Token::Literal(Literal::Int(s.to_string())));

    let char = none_of('\'')
        .delimited_by(just('\''), just('\''))
        .map(|c: char| Token::Literal(Literal::Char(c)));

    let op = one_of("+-*/=<>")
        .repeated()
        .at_least(1)
        .to_slice()
        .map(|s: &str| Token::Op(s.to_string()));

    let punct = one_of("(){}[],.:;").map(Token::Punct);

    let whitespace = one_of(" \t").repeated();

    let newline = text::newline()
        .repeated()
        .at_least(1)
        .map(|_| Token::Newline);

    choice((newline, ident, char, float, int, string, op, punct))
        .map_with(|t, e| (t, e.span()))
        .padded_by(whitespace)
        .repeated()
        .collect()
        .then_ignore(end())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ident, kw,
        lexer::token::punct,
        lexer::token::{newline, Token},
        lit, op,
    };

    fn remove_span(tokens: Vec<Spanned<Token>>) -> Vec<Token> {
        tokens.into_iter().map(|(t, _)| t).collect()
    }

    #[test]
    fn test_lexer() {
        let input = r#"fn main() {
            let x = 42;
            x + 1
        }"#;

        let tokens = remove_span(lexer().parse(input).unwrap());
        assert_eq!(
            tokens,
            vec![
                kw!(fn),
                ident!(main),
                punct('('),
                punct(')'),
                punct('{'),
                newline(),
                kw!(let),
                ident!(x),
                op!(=),
                lit!(42),
                punct(';'),
                newline(),
                ident!(x),
                op!(+),
                lit!(1),
                newline(),
                punct('}'),
            ]
        )
    }
}
