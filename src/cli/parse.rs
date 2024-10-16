use super::build::print_error;
use crate::{
    db::{
        err::Diagnostic,
        input::{Db, SourceDatabase},
    },
    parser::parse_file,
};
use chumsky::Parser;
use salsa::Setter;
use std::{fs, path::Path};

pub fn parse(path: &Path) {
    let mut db = SourceDatabase::default();
    let file = db.input(path);
    file.set_text(&mut db).to(fs::read_to_string(path).unwrap());
    let diags: Vec<Diagnostic> = parse_file::accumulated::<Diagnostic>(&db, file);
    for diag in &diags {
        print_error(&db, diag);
    }
}
