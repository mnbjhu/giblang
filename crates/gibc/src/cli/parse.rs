use crate::{lexer::parser::lexer, parser::file_parser, util::Span};
use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::error::Rich;
use chumsky::input::Input;
use chumsky::Parser;
use std::fmt::Display;
use std::path::Path;

pub fn parse(path: &Path) {
    let text = std::fs::read_to_string(path).unwrap();
    let (tokens, errors) = lexer().parse(&text).into_output_errors();
    let len = text.len();
    for error in errors {
        print(path.to_str().unwrap(), &text, &error, "lex");
    }
    if let Some(tokens) = tokens {
        let eoi = Span::splat(len);
        let input = tokens.spanned(eoi);
        let (ast, errors) = file_parser().parse(input).into_output_errors();
        for error in errors {
            print(path.to_str().unwrap(), &text, &error, "parse");
        }
        if let Some(ast) = ast {
            println!("{ast:#?}");
        }
    }
}

fn print<T: Display>(name: &str, text: &str, error: &Rich<'_, T>, code: &str) {
    let msg = error.reason().to_string();
    let mut builder = Report::build(ReportKind::Error, name, error.span().start)
        .with_code(code)
        .with_message(msg.clone());

    builder = builder.with_label(
        Label::new((name, error.span().into_range()))
            .with_color(Color::Red)
            .with_message(msg),
    );
    builder.finish().print((name, Source::from(text))).unwrap();
}
