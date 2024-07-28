use std::ops::Range;

use ariadne::{Color, Label, Report, ReportKind, Source};

use crate::{parser::File, project::util::path_from_filename, util::Span};

pub struct FileData {
    pub end: u32,
    pub ast: File,
    pub text: String,
    pub name: String,
}

impl FileData {
    pub fn error(&self, text: &str, span: Span) {
        let range: Range<usize> = span.into();
        Report::build(ReportKind::Error, self.name.clone(), span.start)
            .with_label(
                Label::new((self.name.clone(), range))
                    .with_color(Color::Red)
                    .with_message(text),
            )
            .finish()
            .print((self.name.clone(), Source::from(self.text.clone())))
            .unwrap();
    }

    pub fn get_path(&self) -> Vec<String> {
        path_from_filename(&self.name)
    }
}
