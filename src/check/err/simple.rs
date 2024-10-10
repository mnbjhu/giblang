use ariadne::{Color, Source};

use crate::{
    db::input::{Db, SourceFile},
    util::Span,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Simple {
    pub message: String,
    pub span: Span,
    pub file: SourceFile,
}

impl Simple {
    pub fn print(&self, db: &dyn Db) {
        let source = Source::from(self.file.text(db).clone());
        let path = self.file.path(db);
        let name = path.to_str().unwrap();

        let err = Color::Red;

        let mut builder = ariadne::Report::build(ariadne::ReportKind::Error, name, self.span.start)
            .with_message(self.message.to_string())
            .with_code("error");

        builder = builder.with_label(
            ariadne::Label::new((name, self.span.into_range()))
                .with_message(&self.message)
                .with_color(err),
        );

        let report = builder.finish();
        report.print((name, source)).unwrap();
    }
}
