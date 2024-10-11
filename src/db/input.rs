use core::panic;
use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
    vec,
};

use glob::glob;
use salsa::{AsDynDatabase, Database, Setter, Update};
use tracing::info;

use crate::util::Span;

use super::modules::ModulePath;

#[derive(Clone, Default)]
#[salsa::db]
pub struct SourceDatabase {
    storage: salsa::Storage<Self>,
    root: String,
    pub vfs: Option<Vfs>,
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
    fn input(&mut self, path: &Path) -> SourceFile;
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
    fn input(&mut self, path: &Path) -> SourceFile {
        let module_path = get_module_path(self, path);
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
                path.to_path_buf(),
                String::new(),
                module_path.iter().map(|s| (*s).to_string()).collect(),
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

#[must_use]
pub fn module_from_path(path: &Path) -> Vec<&str> {
    path.to_str()
        .unwrap()
        .strip_suffix(".gib")
        .unwrap()
        .split('/')
        .collect()
}

#[salsa::tracked]
impl Vfs {
    #[salsa::tracked]
    pub fn paths(self, db: &dyn Db) -> Vec<PathBuf> {
        match self.inner(db) {
            VfsInner::Dir(dir) => dir.iter().flat_map(|m| m.paths(db)).collect(),
            VfsInner::File(file) => vec![file.path(db)],
        }
    }

    pub fn from_path(db: &mut dyn Db, path: &str) -> Vfs {
        let module = Vfs::new(db, "root".to_string(), VfsInner::Dir(vec![]));
        let pattern = format!("{path}/**/*.gib");
        info!("Searching for files in {}", pattern);
        for file in glob(&pattern).unwrap() {
            let file = file.unwrap();
            if file.is_dir() {
                continue;
            }
            info!("Found file: {}", file.to_string_lossy());
            let src = SourceFile::open(db, file.clone());
            let mut mod_path = get_module_path(db, &file);
            mod_path.pop().unwrap();
            module.insert_path(db.as_dyn_database_mut(), &mod_path, src);
        }
        module
    }

    pub fn get_file(&self, db: &mut dyn Database, path: &[String]) -> Option<SourceFile> {
        let mut module: Vfs = *self;
        for seg in path {
            if let Some(exising) = module.get(db, seg) {
                module = *exising;
            } else {
                let new = Vfs::new(db, (*seg).to_string(), VfsInner::Dir(vec![]));
                module.insert(db, new);
                module = new;
            }
        }
        if let VfsInner::File(source) = module.inner(db) {
            Some(*source)
        } else {
            None
        }
    }

    pub fn insert_path(&self, db: &mut dyn Database, path: &[String], src: SourceFile) {
        let mut module: Vfs = *self;
        for seg in path {
            if let Some(exising) = module.get(db, seg) {
                module = *exising;
            } else {
                let new = Vfs::new(db, (*seg).to_string(), VfsInner::Dir(vec![]));
                module.insert(db, new);
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
            panic!("Get dir called on file {} '{}'", self.name(db), name)
        }
    }

    pub fn insert<'module, 'db: 'module>(&'module mut self, db: &'db mut dyn Database, mod_: Vfs) {
        let new = if let VfsInner::Dir(dir) = self.inner(db) {
            let mut new = dir.clone();
            new.push(mod_);
            new
        } else {
            panic!("Inserted into file")
        };
        self.set_inner(db).to(VfsInner::Dir(new));
    }
}

#[must_use]
pub fn get_path_name(path: &Path) -> String {
    path.file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .strip_suffix(".gib")
        .unwrap()
        .to_string()
}

#[must_use]
pub fn get_module_path(db: &dyn Db, path: &Path) -> Vec<String> {
    path.to_string_lossy()
        .as_ref()
        .strip_prefix(&db.root())
        .unwrap()
        .strip_prefix('/')
        .unwrap()
        .strip_suffix(".gib")
        .unwrap()
        .split('/')
        .map(str::to_string)
        .collect()
}

impl SourceFile {
    pub fn open(db: &dyn Db, path: PathBuf) -> Self {
        let module = get_module_path(db, &path);
        let name = get_path_name(&path);
        let text = read_to_string(path.clone()).unwrap();
        SourceFile::new(db, name, path, text, module)
    }
}
