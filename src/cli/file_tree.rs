use crate::db::input::{Db, SourceDatabase, Vfs, VfsInner};

pub fn file_tree() {
    let pwd = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let mut db = SourceDatabase::default();
    db.init(pwd);
    db.vfs.unwrap().tree(&db, 0);
}

impl Vfs {
    fn tree(self, db: &dyn Db, depth: u32) {
        for _ in 0..depth {
            print!("  ");
        }
        println!("{}", self.name(db));
        if let VfsInner::Dir(children) = &self.inner(db) {
            for child in children {
                child.tree(db, depth + 1);
            }
        }
    }
}
