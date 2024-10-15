use ariadne::{Color, Source};

use crate::{
    db::{
        err::{Diagnostic, Level},
        input::{Db, SourceFile},
    },
    util::Spanned,
};

use super::IntoWithDb;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unresolved {
    pub name: Spanned<String>,
    pub file: SourceFile,
}

impl Unresolved {
    pub fn print(&self, db: &dyn Db) {
        let source = Source::from(self.file.text(db));
        let path = self.file.path(db);
        let name = path.to_str().unwrap();

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

    pub fn message(&self) -> String {
        format!("Unresolved name `{}`", self.name.0)
    }
}

impl IntoWithDb<Diagnostic> for Unresolved {
    fn into_with_db(self, db: &dyn Db) -> Diagnostic {
        Diagnostic {
            message: self.message(),
            span: self.name.1,
            level: Level::Error,
            path: self.file.path(db),
        }
    }
}
