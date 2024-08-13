use ariadne::{Color, Source};

use crate::{project::Project, util::Spanned};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unresolved {
    pub name: Spanned<String>,
    pub file: u32,
}

impl Unresolved {
    pub fn print(&self, project: &Project) {
        let file_data = project
            .get_file(self.file)
            .unwrap_or_else(|| panic!("No file found for id {}", self.file));
        let source = Source::from(file_data.text.clone());
        let name = &file_data.name;

        let err = Color::Red;

        let mut builder =
            ariadne::Report::build(ariadne::ReportKind::Error, name, self.name.1.start)
                .with_message(format!("Unresolved name `{}`", self.name.0))
                .with_code("error");

        builder = builder.with_label(
            ariadne::Label::new((name, self.name.1.into_range()))
                .with_message(format!("Unresolved name `{}`", self.name.0))
                .with_color(err),
        );

        let report = builder.finish();
        report.print((name, source)).unwrap();
    }
}
