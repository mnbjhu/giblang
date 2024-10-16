use ariadne::{Color, Source};

use crate::{
    db::{
        err::{Diagnostic, Level},
        input::{Db, SourceFile},
    },
    util::Span,
};

use super::IntoWithDb;

#[derive(Clone, Debug)]
pub struct ImplTypeMismatch {
    pub found: String,
    pub span: Span,
    pub file: SourceFile,
}

impl ImplTypeMismatch {
    pub fn print(&self, db: &dyn Db) {
        let message = self.message();
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

    pub fn message(&self) -> String {
        format!(
            "Expected type to be a named type but found `{}`",
            self.found
        )
    }
}

impl IntoWithDb<Diagnostic> for ImplTypeMismatch {
    fn into_with_db(self, db: &dyn Db) -> Diagnostic {
        Diagnostic {
            message: self.message(),
            span: self.span,
            level: Level::Error,
            path: self.file.path(db),
            file: self.file,
            expected: vec![],
        }
    }
}
