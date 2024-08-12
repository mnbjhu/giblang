use ariadne::{Color, Source};

use crate::{check::state::CheckState, util::Span};

#[derive(Clone, Debug, PartialEq)]
pub struct UnboundTypeVar {
    pub span: Span,
    pub file: u32,
    pub name: String,
}

impl UnboundTypeVar {
    pub fn print(&self, state: &CheckState) {
        let file_data = state
            .project
            .get_file(self.file)
            .unwrap_or_else(|| panic!("No file found for id {}", self.file));
        let source = Source::from(file_data.text.clone());
        let name = &file_data.name;

        let err = Color::Red;
        let msg = format!("Cannot imply type for type generic parameter '{name}'",);

        let mut builder = ariadne::Report::build(ariadne::ReportKind::Error, name, self.span.start)
            .with_message(msg.clone())
            .with_code("error");

        builder = builder.with_label(
            ariadne::Label::new((name, self.span.into_range()))
                .with_message(&msg)
                .with_color(err),
        );

        let report = builder.finish();
        report.print((name, source)).unwrap();
    }
}
