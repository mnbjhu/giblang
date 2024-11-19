use crate::{
    check::{check_project, check_vfs, resolve_project},
    db::{err::Diagnostic, input::SourceDatabase},
    run::state::ProgramState,
};

use super::build::print_error;

pub fn debug() {
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
        let file = db.vfs.unwrap().build(&db, project);
        let mut prog = ProgramState::new();
        prog.vtables = file.tables;
        prog.run_debug(&file.funcs);
    }
}
