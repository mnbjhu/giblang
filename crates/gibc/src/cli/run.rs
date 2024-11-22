use gvm::{format::ByteCodeFile, vm::state::ProgramState};

use crate::{
    check::{build_state::BuildState, check_file, check_project, check_vfs, resolve_project},
    db::{
        decl::Project,
        err::Diagnostic,
        input::{Db, SourceDatabase, Vfs, VfsInner},
    },
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
        let file = db.vfs.unwrap().build(&db, project);
        let mut prog = ProgramState::new(&file.funcs, file.tables, file.file_names);
        prog.run();
    }
}

impl<'db> Vfs {
    pub fn build(self, db: &'db dyn Db, project: Project<'db>) -> ByteCodeFile {
        match self.inner(db) {
            VfsInner::Dir(files) => {
                let mut code = ByteCodeFile::default();
                for file in files {
                    let file_code = file.build(db, project);
                    code.funcs.extend(file_code.funcs);
                    code.tables.extend(file_code.tables);
                    code.file_names.extend(file_code.file_names);
                }
                code
            }
            VfsInner::File(file) => {
                let ir = check_file(db, *file, project);
                let mut state = BuildState::new(db, project, *file);
                ir.build(&mut state)
            }
        }
    }
}
