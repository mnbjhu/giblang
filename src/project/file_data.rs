use std::ops::Range;

use ariadne::{Label, Report, ReportKind, Source};

use crate::{parser::File, util::Span};

pub struct FileData {
    pub end: u32,
    pub ast: File,
    pub text: String,
    pub name: String,
}

impl FileData {
    pub fn print_report(&self, report: Report<(&String, Range<usize>)>) {
        let source = Source::from(self.text);
        let name = &self.name;
        report.print((name, source)).unwrap();
    }

    pub fn error(&self, text: &str, span: Span) {
        let range: Range<usize> = span.into();
        Report::build(ReportKind::Error, self.name.clone(), span.start)
            .with_label(Label::new((self.name.clone(), range)).with_message(text))
            .finish()
            .print((self.name.clone(), Source::from(self.text.clone())))
            .unwrap();
    }
}
