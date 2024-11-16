use std::collections::HashMap;

use crate::{
    check::{build_state::BuildState, check_file, check_project, check_vfs, resolve_project},
    db::{
        decl::Project,
        err::Diagnostic,
        input::{Db, SourceDatabase, Vfs, VfsInner},
    },
    run::state::{FuncDef, ProgramState},
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
        let funcs = db.vfs.unwrap().build(&db, project);
        let main = funcs.get(&0).expect("no main function");
        let mut prog = ProgramState::new(&main.body, 0);
        prog.run(&funcs);
    }
}

impl<'db> Vfs {
    pub fn build(self, db: &'db dyn Db, project: Project<'db>) -> HashMap<u32, FuncDef> {
        match self.inner(db) {
            VfsInner::Dir(files) => {
                let mut funcs = HashMap::new();
                for file in files {
                    let file_funcs = file.build(db, project);
                    funcs.extend(file_funcs);
                }
                funcs
            }
            VfsInner::File(file) => {
                let ir = check_file(db, *file, project);
                let mut state = BuildState::new(db);
                ir.build(&mut state)
            }
        }
    }
}
