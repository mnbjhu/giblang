use core::panic;
use std::{fs::read_to_string, path::PathBuf, vec};

use glob::glob;
use salsa::{Database, Setter, Update};

use crate::util::Span;

#[derive(Clone)]
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
pub struct Dir {
    #[return_ref]
    pub name: String,

    #[return_ref]
    modules: Vec<Module>,
}

#[derive(Debug, Update, Clone)]
pub enum Module {
    File(SourceFile),
    Dir(Dir),
}

impl Module {
    pub fn from_path(db: &dyn Database, path: &str) -> Self {
        let module = Module::Dir(Dir::new(db, "root".to_string(), vec![]));
        for _ in glob(path).unwrap() {
            todo!()
        }
        module
    }

    pub fn name<'db>(&self, db: &'db dyn Database) -> &'db str {
        match self {
            Module::File(f) => f.name(db),
            Module::Dir(d) => d.name(db),
        }
    }

    pub fn get_or_add(&self, db: &mut dyn Database, path: &[&str], src: SourceFile) {
        let mut module: Module = self.clone();
        for seg in path {
            if let Some(exising) = module.get(db, seg) {
                module = exising.clone();
            } else {
                let new = Module::Dir(Dir::new(db, (*seg).to_string(), vec![]));
                module.insert(db, new.clone());
                module = new;
            }
        }
        module.insert(db, Module::File(src));
    }

    pub fn file_set<'db>(&self, db: &'db dyn Database) -> Vec<&'db SourceFile> {
        if let Module::Dir(dir) = self {
            dir.modules(db)
                .iter()
                .filter_map(|mod_| {
                    if let Module::File(f) = mod_ {
                        Some(f)
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            panic!("Files set called on file")
        }
    }

    pub fn get<'db>(&self, db: &'db dyn Database, name: &str) -> Option<&'db Module> {
        if let Module::Dir(dir) = self {
            Some(dir.get(db, name)?)
        } else {
            panic!("Get dir called on file")
        }
    }

    pub fn insert(&self, db: &mut dyn Database, mod_: Module) {
        if let Module::Dir(dir) = self {
            let mut new = dir.modules(db).clone();
            new.push(mod_);
            dir.set_modules(db).to(new);
        } else {
            panic!("Inserted into file")
        }
    }
}

impl Dir {
    pub fn get<'db>(&self, db: &'db dyn Database, name: &str) -> Option<&'db Module> {
        self.modules(db).iter().find(|mod_| mod_.name(db) == name)
    }

    pub fn get_dir<'db>(&self, db: &'db dyn Database, name: &str) -> Option<&'db Dir> {
        if let Some(Module::Dir(dir)) = self.get(db, name) {
            Some(dir)
        } else {
            None
        }
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
