use ariadne::{Color, Source};

use crate::{
    db::input::{Db, SourceFile},
    util::Spanned,
};

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
}
