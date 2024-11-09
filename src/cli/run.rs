use crate::{
    check::{check_project, check_vfs, resolve_project},
    db::{decl::DeclKind, err::Diagnostic, input::SourceDatabase, path::ModulePath},
};

use super::build::print_error;

pub fn run() {
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
        let main = project.get_decl(
            &db,
            ModulePath::new(&db, vec!["main".to_string(), "main".to_string()]),
        ).expect("Expected a main function at `main::main`");
        let DeclKind::Function(main) = main.kind(&db) else {
            panic!("Expected `main::main` to be a function")
        };
    }
}
