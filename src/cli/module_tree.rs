use crate::{
    db::{
        input::{Db, SourceDatabase},
        modules::{Module, ModuleData},
    },
    project::decl::DeclKind,
    resolve::resolve_vfs,
};

pub fn module_tree() {
    let pwd = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let mut db = SourceDatabase::default();
    db.init(pwd);
    let module = resolve_vfs(&db, db.vfs.unwrap());
    module.tree(&db, 0);
}

impl<'db> Module<'db> {
    fn tree(self, db: &'db dyn Db, depth: u32) {
        for _ in 0..depth {
            print!("  ");
        }
        println!("{}", self.name(db));
        match &self.content(db) {
            ModuleData::Package(children) => {
                for child in children {
                    child.tree(db, depth + 1);
                }
            }
            ModuleData::Export(export) => match export.kind(db) {
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
                _ => (),
            },
        }
    }
}
