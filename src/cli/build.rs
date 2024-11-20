use std::fs;
use std::io::Write;

use ariadne::{Color, Source};
use ariadne::{Label, Report, ReportKind};
use tracing_subscriber::fmt::MakeWriter;

use crate::check::{check_project, check_vfs, resolve_project};
use crate::db::err::Diagnostic;
use crate::db::input::{Db, SourceDatabase};
use crate::run::bin::encode_program;
use crate::run::bytecode::ByteCode;
use crate::run::text::write_text;

pub fn build() {
    let pwd = std::env::current_dir().unwrap();
    let mut db = SourceDatabase::default();
    db.init(pwd.to_string_lossy().to_string());
    let project = resolve_project(&db, db.vfs.unwrap());
    check_vfs(&db, db.vfs.unwrap(), project);
    let diags: Vec<Diagnostic> = check_project::accumulated::<Diagnostic>(&db, db.vfs.unwrap());
    for diag in &diags {
        print_error(&db, diag);
    }
    if diags.is_empty() {
        let out_file = pwd.join("out.txt");
        let mut out = fs::File::create(out_file.clone())
            .or_else(|_| {
                fs::remove_file(out_file.clone()).unwrap();
                fs::File::create(out_file)
            })
            .unwrap();
        let mut file = db.vfs.unwrap().build(&db, project);
        for func in file.funcs.values_mut() {
            while let Some((index, (line, col))) = func.marks.pop() {
                func.body.insert(index, ByteCode::Mark(line, col));
            }
        }
        write_text(&file, &mut out.make_writer());
        let out_file = pwd.join("out");
        let bytes = encode_program(&file.funcs, &file.tables, &file.file_names);
        let mut out = fs::File::create(out_file.clone())
            .or_else(|_| {
                fs::remove_file(out_file.clone()).unwrap();
                fs::File::create(out_file)
            })
            .unwrap();
        out.write_all(&bytes).unwrap();
    }
}

pub fn print_error(db: &dyn Db, error: &Diagnostic) {
    let source = Source::from(error.file.text(db));
    let red = Color::Red;

    let name = error
        .path
        .to_str()
        .unwrap()
        .strip_prefix(&db.root())
        .unwrap()
        .strip_prefix('/')
        .unwrap();
    let mut builder = Report::build(ReportKind::Error, name, error.span.start)
        // .with_code(code)
        .with_message(error.message.to_string());

    builder = builder.with_label(
        Label::new((name, error.span.into_range()))
            .with_color(red)
            .with_message(error.message.to_string()),
    );
    let report = builder.finish();
    report.print((name, source)).unwrap();
}
