use ariadne::{Color, Source};

use crate::{
    db::input::{Db, SourceFile},
    util::Span,
};

#[derive(Clone, Debug)]
pub struct ImplTypeMismatch {
    pub found: String,
    pub span: Span,
    pub file: SourceFile,
}

impl ImplTypeMismatch {
    pub fn print(&self, db: &dyn Db) {
        let message = format!(
            "Expected type to be a named type but found `{}`",
            self.found
        );
        let source = Source::from(self.file.text(db).clone());
        let path = self.file.path(db);
        let name = path.to_str().unwrap();

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
