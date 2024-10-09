use core::panic;
use std::{fs::read_to_string, path::PathBuf, vec};

use glob::glob;
use salsa::{Database, Setter, Update};

use crate::util::Span;

#[derive(Clone, Default)]
#[salsa::db]
pub struct SourceDatabase {
    storage: salsa::Storage<Self>,
}

#[salsa::db]
impl salsa::Database for SourceDatabase {
    fn salsa_event(&self, _: &dyn Fn() -> salsa::Event) {}
}

#[salsa::accumulator]
pub struct Diagnostic {
    pub message: String,
    pub span: Span,
    pub level: Level,
    pub path: PathBuf,
}

#[derive(Clone, Debug)]
pub enum Level {
    Error,
    Warning,
}

#[salsa::input]
pub struct SourceFile {
    #[return_ref]
    pub name: String,

    #[interned]
    pub path: PathBuf,

    #[return_ref]
    pub text: String,
}

#[salsa::input]
pub struct Vfs {
    pub name: String,
    #[return_ref]
    pub inner: VfsInner,
}

#[derive(Debug, Update, Clone)]
pub enum VfsInner {
    File(SourceFile),
    Dir(Vec<Vfs>),
}

#[salsa::tracked]
impl Vfs {
    pub fn from_path(db: &mut dyn Database, path: &str) -> Vfs {
        let module = Vfs::new(db, "root".to_string(), VfsInner::Dir(vec![]));
        for file in glob(path).unwrap() {
            let file = file.unwrap();
            let src = SourceFile::open(db, file.clone());
            let mut mod_path = file
                .to_str()
                .unwrap()
                .strip_suffix(".gib")
                .unwrap()
                .split('/')
                .collect::<Vec<&str>>();
            mod_path.pop().unwrap();
            module.insert_path(db.as_dyn_database_mut(), &mod_path, src);
        }
        module
    }

    pub fn insert_path(&self, db: &mut dyn Database, path: &[&str], src: SourceFile) {
        let mut module: Vfs = self.clone();
        for seg in path {
            if let Some(exising) = module.get(db, seg) {
                module = exising.clone();
            } else {
                let new = Vfs::new(db, (*seg).to_string(), VfsInner::Dir(vec![]));
                module.insert(db, new.clone());
                module = new;
            }
        }
        module.set_inner(db).to(VfsInner::File(src));
    }

    pub fn get<'module, 'db: 'module>(
        &'module self,
        db: &'db dyn Database,
        name: &str,
    ) -> Option<&'module Vfs> {
        if let VfsInner::Dir(dir) = self.inner(db) {
            dir.iter().find(|mod_| mod_.name(db) == name)
        } else {
            panic!("Get dir called on file")
        }
    }

    pub fn insert<'module, 'db: 'module>(&'module mut self, db: &'db mut dyn Database, mod_: Vfs) {
        let new = if let VfsInner::Dir(dir) = self.inner(db) {
            let mut new = dir.to_vec();
            new.push(mod_);
            new
        } else {
            panic!("Inserted into file")
        };
        self.set_inner(db).to(VfsInner::Dir(new));
    }
}

impl SourceFile {
    pub fn open(db: &dyn Database, path: PathBuf) -> Self {
        let name = path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .strip_suffix(".gib")
            .unwrap()
            .to_string();
        let text = read_to_string(path.clone()).unwrap();
        SourceFile::new(db, name, path, text)
    }
}
