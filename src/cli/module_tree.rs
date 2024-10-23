use crate::{
    check::resolve_project,
    db::{decl::{Decl, DeclKind}, input::{Db, SourceDatabase}},
};

pub fn module_tree() {
    let pwd = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let mut db = SourceDatabase::default();
    db.init(pwd);
    let project = resolve_project(&db, db.vfs.unwrap());
    project.decls(&db).tree(&db, 0);
}

impl<'db> Decl<'db> {
    fn tree(self, db: &'db dyn Db, depth: u32) {
        for _ in 0..depth {
            print!("  ");
        }
        println!("{}", self.name(db));
        match self.kind(db) {
            DeclKind::Trait { body, .. } => {
                for child in body {
                    child.tree(db, depth + 1);
                }
            }
            DeclKind::Enum { variants, .. } => {
                for child in variants {
                    child.tree(db, depth + 1);
                }
            }
            DeclKind::Module(children) => {
                for child in children {
                    child.tree(db, depth + 1);
                }
            }
            _ => (),
        }
    }
}
