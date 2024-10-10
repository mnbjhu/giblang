use ariadne::{Color, Source};

use crate::{
    db::input::{Db, SourceFile},
    util::Span,
};

#[derive(Clone, Debug, PartialEq)]
pub struct UnboundTypeVar {
    pub span: Span,
    pub file: SourceFile,
    pub name: String,
}

impl UnboundTypeVar {
    pub fn print(&self, db: &dyn Db) {
        let source = Source::from(self.file.text(db));
        let path = self.file.path(db);
        let name = path.to_str().unwrap();

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
