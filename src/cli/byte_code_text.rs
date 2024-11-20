use std::fmt::Display;
use std::{fs, path::Path};

use crate::run::state::ProgramState;
use crate::run::text::{byte_code_file_parser, byte_code_lexer};
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
        let (funcs, errors) = byte_code_file_parser()
            .parse(parser_input)
            .into_output_errors();
        for err in errors {
            print_error(&err, &text, path);
        }
        if let Some(bc_file) = funcs {
            let mut prog = ProgramState::new(&bc_file.funcs, bc_file.tables, bc_file.file_names);
            prog.run();
        }
    }
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
