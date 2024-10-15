use ariadne::{Color, Source};

use crate::{
    db::{
        err::{Diagnostic, Level},
        input::{Db, SourceFile},
    },
    util::Span,
};

use super::IntoWithDb;

#[derive(Clone, Debug, PartialEq)]
pub struct IsNotInstance {
    pub found: String,
    pub expected: String,
    pub span: Span,
    pub file: SourceFile,
}

impl IsNotInstance {
    pub fn print(&self, db: &dyn Db) {
        let source = Source::from(self.file.text(db).clone());
        let path = self.file.path(db);
        let name = path.to_str().unwrap();

        let err = Color::Red;
        let msg = self.message();

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

    pub fn message(&self) -> String {
        format!("Expected {} but found {}", self.expected, self.found)
    }
}

impl IntoWithDb<Diagnostic> for IsNotInstance {
    fn into_with_db(self, db: &dyn Db) -> Diagnostic {
        Diagnostic {
            message: self.message(),
            span: self.span,
            level: Level::Error,
            path: self.file.path(db),
        }
    }
}
