use core::panic;
use std::{fs::read_to_string, path::PathBuf, vec};

use glob::glob;
use salsa::{AsDynDatabase, Database, Setter, Storage, Update};

use crate::util::Span;

use super::modules::ModulePath;

#[derive(Clone, Default)]
#[salsa::db]
pub struct SourceDatabase {
    storage: salsa::Storage<Self>,
    root: String,
    vfs: Option<Vfs>,
}

impl SourceDatabase {
    pub fn init(&mut self, root: String) {
        let vfs = Vfs::from_path(self, &root);
        self.vfs = Some(vfs);
        self.root = root;
    }
}

#[salsa::db]
pub trait Db: salsa::Database {
    fn root(&self) -> String;
    fn input(&mut self, path: &PathBuf) -> SourceFile;
}

#[salsa::db]
impl salsa::Database for SourceDatabase {
    fn salsa_event(&self, _: &dyn Fn() -> salsa::Event) {}
}

#[salsa::db]
impl Db for SourceDatabase {
    fn root(&self) -> String {
        self.root.to_string()
    }
    fn input(&mut self, path: &PathBuf) -> SourceFile {
        let module_path = module_from_path(path);
        let file = self
            .vfs
            .unwrap()
            .get_file(self.as_dyn_database_mut(), &module_path);
        if let Some(existing) = file {
            existing
        } else {
            let src = SourceFile::new(
                self.as_dyn_database(),
                get_path_name(path),
                path.clone(),
                "".to_string(),
                module_path.iter().map(|s| s.to_string()).collect(),
            );
            self.vfs
                .unwrap()
                .insert_path(self.as_dyn_database_mut(), &module_path, src);
            let file = self
                .vfs
                .unwrap()
                .get_file(self.as_dyn_database_mut(), &module_path);
            file.unwrap()
        }
    }
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

    #[interned]
    pub module: Vec<String>,
}

impl SourceFile {
    pub fn module_path<'db>(&self, db: &'db dyn Db) -> ModulePath<'db> {
        ModulePath::new(db, self.module(db))
    }
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

pub fn module_from_path<'path>(path: &'path PathBuf) -> Vec<&'path str> {
    path.to_str()
        .unwrap()
        .strip_suffix(".gib")
        .unwrap()
        .split('/')
        .collect()
}

#[salsa::tracked]
impl Vfs {
    pub fn from_path(db: &mut dyn Db, path: &str) -> Vfs {
        let module = Vfs::new(db, "root".to_string(), VfsInner::Dir(vec![]));
        for file in glob(path).unwrap() {
            let file = file.unwrap();
            if file.is_dir() {
                continue;
            }
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

    pub fn get_file(&self, db: &mut dyn Database, path: &[&str]) -> Option<SourceFile> {
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
        if let VfsInner::File(source) = module.inner(db) {
            Some(*source)
        } else {
            None
        }
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

pub fn get_path_name(path: &PathBuf) -> String {
    path.file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .strip_suffix(".gib")
        .unwrap()
        .to_string()
}

impl SourceFile {
    pub fn open(db: &dyn Db, path: PathBuf) -> Self {
        let module = path
            .to_string_lossy()
            .as_ref()
            .strip_prefix(db.root().as_str())
            .unwrap()
            .strip_suffix(".gib")
            .unwrap_or_else(|| panic!("Path doesn't end with .gib '{}'", path.to_string_lossy()))
            .split('/')
            .map(str::to_string)
            .collect::<Vec<_>>();
        let name = get_path_name(&path);
        let text = read_to_string(path.clone()).unwrap();
        SourceFile::new(db, name, path, text, module)
    }
}
