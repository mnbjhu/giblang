use std::fmt::Display;

use ariadne::{Color, Fmt, Source};
use ariadne::{ColorGenerator, Label, Report, ReportKind};
use chumsky::error::Rich;

use crate::project::Project;

pub fn build() {
    let mut project = Project::init_pwd();
    let errors = project.resolve();
    for error in &errors {
        project.print_error(error);
    }
    let errors = project.check();
    for error in &errors {
        project.print_error(error);
    }
}

pub fn print_error<T: Display>(error: &Rich<'_, T>, source: Source, name: &str, code: &str) {
    let mut colors = ColorGenerator::new();

    let b = colors.next();
    let out = Color::Fixed(81);

    let mut builder = Report::build(ReportKind::Error, name, error.span().start)
        .with_code(code)
        .with_message(error.reason().to_string());

    if let Some(found) = error.found() {
        builder = builder.with_label(
            Label::new((name, error.span().into_range()))
                .with_message(format!("Found {}", found))
                .with_color(b),
        );
    } else {
        builder = builder.with_label(
            Label::new((name, error.span().into_range()))
                .with_color(b)
                .with_message(error.reason().to_string()),
        );
    }

    let expected = error.expected().map(|e| e.to_string()).collect::<Vec<_>>();
    if !expected.is_empty() {
        builder = builder.with_note(
            format!(
                "Expected {}",
                error
                    .expected()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(" or ")
            )
            .fg(out),
        );
    }
    let report = builder.finish();
    report.print((name, source.clone())).unwrap();
}
