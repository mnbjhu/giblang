use std::collections::HashMap;
use std::fmt::Display;
use std::{fs, path::Path};

use crate::run::state::{FuncDef, ProgramState};
use crate::run::text::{bc_parser, byte_code_lexer};
use crate::util::Span;
use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::error::Rich;
use chumsky::input::Input as _;
use chumsky::Parser as _;

#[allow(clippy::cast_possible_truncation)]
pub fn byte_code_text(path: &Path) {
    let text = fs::read_to_string(path).unwrap();
    let (tokens, errors) = byte_code_lexer().parse(&text).into_output_errors();
    for err in errors {
        print_error(&err, &text, path);
    }
    if let Some(tokens) = tokens {
        let parser_input = tokens.spanned(Span::splat(text.len()));
        let (funcs, errors) = bc_parser().parse(parser_input).into_output_errors();
        for err in errors {
            print_error(&err, &text, path);
        }
        if let Some(funcs) = funcs {
            let mut prog = ProgramState::new();
            prog.run(&funcs);
        }
    }
}

pub fn parse_byte_code_text(text: &str) -> Option<HashMap<u32, FuncDef>> {
    let tokens = byte_code_lexer().parse(&text).into_output()?;
    let parser_input = tokens.spanned(Span::splat(text.len()));
    let funcs = bc_parser().parse(parser_input).into_output()?;
    Some(funcs)
}

pub fn print_error<T: Display>(error: &Rich<'_, T>, text: &str, path: &Path) {
    let source = Source::from(text);
    let red = Color::Red;
    let name = path.to_str().unwrap().to_string();

    let mut builder = Report::build(ReportKind::Error, name.clone(), error.span().start)
        // .with_code(code)
        .with_message(error.reason().to_string());

    builder = builder.with_label(
        Label::new((name.clone(), (*error.span()).into()))
            .with_color(red)
            .with_message(error.reason().to_string()),
    );
    let report = builder.finish();
    report.print((name, source)).unwrap();
}
