
use ariadne::Source;
use ariadne::{ColorGenerator, Label, Report, ReportKind};

use crate::check::check_project;
use crate::db::err::Diagnostic;
use crate::db::input::{Db, SourceDatabase};

pub fn build() {
    let pwd = std::env::current_dir().unwrap();
    let mut db = SourceDatabase::default();
    db.init(pwd.to_string_lossy().to_string());
    let diags: Vec<Diagnostic> = check_project::accumulated::<Diagnostic>(&db, db.vfs.unwrap());
    for diag in diags.iter() {
        print_error(&db, diag);
    }
}

pub fn print_error(db: &dyn Db, error: &Diagnostic) {
    let mut colors = ColorGenerator::new();
    let source = Source::from(error.file.text(db));

    let b = colors.next();

    let name = error.path.to_str().unwrap();
    let mut builder = Report::build(ReportKind::Error, name, error.span.start)
        // .with_code(code)
        .with_message(error.message.to_string());

    builder = builder.with_label(
        Label::new((name, error.span.into_range()))
            .with_color(b)
            .with_message(error.message.to_string()),
    );
    let report = builder.finish();
    report.print((name, source)).unwrap();
}
