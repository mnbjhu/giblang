use std::ops::Range;

use crate::lexer::{literal::Literal, parser::lexer, token::Token};
use ariadne::{Color, ColorGenerator, Fmt as _, Label, Report, ReportKind, Source};
use chumsky::{error::Rich, Parser};

pub fn lex(path: &str) {
    let source = std::fs::read_to_string(path).unwrap();
    let (tokens, errors) = lexer().parse(&source).into_output_errors();
    for error in errors {
        let report = build_lex_report("input", error);
        report
            .print(("input", Source::from(&source)))
            .expect("failed to print report");
    }
    if let Some(tokens) = tokens {
        let mut colors = ColorGenerator::new();
        let keyword = colors.next();
        let punct = colors.next();
        let ident = colors.next();
        let op = colors.next();
        let newline = colors.next();
        let string = colors.next();
        let int = colors.next();
        let float = colors.next();
        let char = colors.next();
        let bool = colors.next();

        let mut cursor = 0;
        for (token, span) in tokens {
            let span: Range<usize> = span.into();
            if span.start > cursor {
                let s = &source[cursor..span.start];
                print!("{}", s);
                cursor = span.start;
            }
            let s = &source[span.clone()];
            let color = match token {
                Token::Keyword(_) => keyword,
                Token::Punct(_) => punct,
                Token::Ident(_) => ident,
                Token::Op(_) => op,
                Token::Newline => newline,
                Token::Literal(Literal::String(_)) => string,
                Token::Literal(Literal::Int(_)) => int,
                Token::Literal(Literal::Float(_)) => float,
                Token::Literal(Literal::Char(_)) => char,
                Token::Literal(Literal::Bool(_)) => bool,
            };
            if let Token::Newline = token {
                print!("{}", s[0..s.len() - 1].fg(color));
                println!("↵");
            } else {
                print!("{}", s.fg(color));
            }
            cursor = span.end;
        }
    }
}

pub fn build_lex_report<'a>(
    filename: &'a str,
    from: Rich<'a, char>,
) -> ariadne::Report<'a, (&'a str, std::ops::Range<usize>)> {
    let error = Color::Fixed(1);
    Report::build(ReportKind::Error, filename, from.span().start)
        .with_code(1)
        .with_message("Lexical error")
        .with_label(
            Label::new((filename, (*from.span()).into()))
                .with_message(from.reason().clone())
                .with_color(error),
        )
        .finish()
}
