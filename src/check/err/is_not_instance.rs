use ariadne::{Color, Source};

use crate::{check::state::CheckState, project::Project, ty::Ty, util::Span};

#[derive(Clone, Debug, PartialEq)]
pub struct IsNotInstance {
    pub first: Ty,
    pub second: Ty,
    pub span: Span,
    pub file: u32,
}

impl IsNotInstance {
    pub fn print(&self, state: &CheckState) {
        let file_data = state
            .project
            .get_file(self.file)
            .unwrap_or_else(|| panic!("No file found for id {}", self.file));
        let source = Source::from(file_data.text.clone());
        let name = &file_data.name;

        let err = Color::Red;
        let msg = format!(
            "Expected {} but found {}",
            self.second.get_name(state),
            self.first.get_name(state),
        );

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
