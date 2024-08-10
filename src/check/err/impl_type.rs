use ariadne::{Color, Source};

use crate::{project::Project, util::Span};

pub struct ImplTypeMismatch {
    pub found: String,
    pub span: Span,
    pub file: u32,
}

impl ImplTypeMismatch {
    pub fn print(&self, project: &Project) {
        let message = format!(
            "Expected type to be a named type but found `{}`",
            self.found
        );
        let file_data = project
            .get_file(self.file)
            .unwrap_or_else(|| panic!("No file found for id {}", self.file));
        let source = Source::from(file_data.text.clone());
        let name = &file_data.name;

        let err = Color::Red;

        let mut builder = ariadne::Report::build(ariadne::ReportKind::Error, name, self.span.start)
            .with_message(&message)
            .with_code("error");

        builder = builder.with_label(
            ariadne::Label::new((name, self.span.into_range()))
                .with_message(&message)
                .with_color(err),
        );

        let report = builder.finish();
        report.print((name, source)).unwrap();
    }
}
