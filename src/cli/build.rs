use std::{fmt::Display, fs};

use ariadne::Source;
use chumsky::{error::Rich, input::Input, Parser};

use crate::{lexer::parser::lexer, parser::file_parser, util::Span};

pub fn build(path: &str) {
    let src = fs::read_to_string(path).unwrap();
    let eoi = Span::splat(src.len());
    let (tokens, errors) = lexer().parse(&src).into_output_errors();
    let source = Source::from(src.clone());

    for error in errors {
        print_error(error, &source, path, "Lexer Error");
    }

    if let Some(tokens) = tokens {
        let input = tokens.spanned(eoi);
        let (ast, errors) = file_parser().parse(input).into_output_errors();
        for error in errors {
            print_error(error, &source, path, "Parser Error");
        }

        println!("{:#?}", ast);
    }
}

pub fn print_error<T: Display>(error: Rich<'_, T>, source: &Source, name: &str, code: &str) {
    use ariadne::{Color, ColorGenerator, Fmt, Label, Report, ReportKind};

    let mut colors = ColorGenerator::new();

    // Generate & choose some colours for each of our elements
    // let a = colors.next();
    let b = colors.next();
    let out = Color::Fixed(81);

    let found = error
        .found()
        .map(|e| e.to_string())
        .unwrap_or("EOF".to_string());

    Report::build(ReportKind::Error, name, error.span().start)
        .with_code(code)
        .with_message(error.reason().to_string())
        .with_label(
            Label::new((name, error.span().into_range()))
                .with_message(format!("Found {}", found))
                .with_color(b),
        )
        .with_note(
            format!(
                "Expected {}",
                error
                    .expected()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(" or ")
            )
            .fg(out),
        )
        .finish()
        .print((name, source.clone()))
        .unwrap();
}
