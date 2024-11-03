use core::panic;
use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
    str::FromStr,
    vec,
};

use glob::glob;
use salsa::{AsDynDatabase, Setter, Update};

use super::path::ModulePath;

#[derive(Clone, Default)]
#[salsa::db]
pub struct SourceDatabase {
    storage: salsa::Storage<Self>,
    root: String,
    pub vfs: Option<Vfs>,
}

impl SourceDatabase {
    pub fn init(&mut self, root: String) {
        self.root = root;
        let vfs = Vfs::from_path(self);
        self.vfs = Some(vfs);
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
        let file = self.vfs.unwrap().get_file(self, &module_path);
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
            self.vfs.unwrap().insert_path(self, &module_path, src);
            let file = self.vfs.unwrap().get_file(self, &module_path);
            file.unwrap()
        }
    }
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
    pub fn module_path(self, db: &dyn Db) -> ModulePath<'_> {
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

#[salsa::tracked]
impl Vfs {
    #[salsa::tracked]
    pub fn paths(self, db: &dyn Db) -> Vec<PathBuf> {
        match self.inner(db) {
            VfsInner::Dir(dir) => dir.iter().flat_map(|m| m.paths(db)).collect(),
            VfsInner::File(file) => vec![file.path(db)],
        }
    }

    pub fn from_path(db: &mut dyn Db) -> Vfs {
        let std = Vfs::new(
            db,
            "std".to_string(),
            VfsInner::File(SourceFile::new(
                db,
                "std".to_string(),
                PathBuf::from_str(&format!("{root}/std.gib", root = db.root())).unwrap(),
                r"
                struct Int
                struct Float
                struct Bool
                struct String
                struct Any

                fn panic(message: String): Nothing
                fn print(text: String)
                fn println(text: String)

                struct Vec[T]

                impl[T] Vec[T] {
                    fn new(): Self {
                        Vec
                    }
                    fn Self.push(item: T)
                    fn Self.get(index: Int): Option[T]
                }

                enum Option[T] {
                    Some(T),
                    None
                }

                enum Result[R, E] {
                    Ok(R),
                    Err(E),
                }
                "
                .to_string(),
                vec!["std".to_string()],
            )),
        );
        let module = Vfs::new(db, "root".to_string(), VfsInner::Dir(vec![std]));
        let pattern = format!("{path}/**/*.gib", path = db.root());

        for file in glob(&pattern).unwrap() {
            let file = file.unwrap();
            if file.is_dir() {
                continue;
            }
            let src = SourceFile::open(db, file.clone());
            let mod_path = get_module_path(db, &file);
            module.insert_path(db, &mod_path, src);
        }
        module
    }

    pub fn get_file(self, db: &mut dyn Db, path: &[String]) -> Option<SourceFile> {
        let mut module: Vfs = self;
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

    pub fn insert_path(self, db: &mut dyn Db, path: &[String], src: SourceFile) {
        let mut module: Vfs = self;
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
        db: &'db dyn Db,
        name: &str,
    ) -> Option<&'module Vfs> {
        if let VfsInner::Dir(dir) = self.inner(db) {
            dir.iter().find(|mod_| mod_.name(db) == name)
        } else {
            panic!("Get dir called on file {} '{}'", self.name(db), name)
        }
    }

    pub fn insert<'module, 'db: 'module>(&'module mut self, db: &'db mut dyn Db, mod_: Vfs) {
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
    let res = path
        .to_string_lossy()
        .as_ref()
        .strip_prefix(&db.root())
        .unwrap()
        .strip_prefix('/')
        .unwrap()
        .strip_suffix(".gib")
        .unwrap()
        .split('/')
        .map(str::to_string)
        .collect();
    res
}

impl SourceFile {
    pub fn open(db: &dyn Db, path: PathBuf) -> Self {
        let module = get_module_path(db, &path);
        let name = get_path_name(&path);
        let text = read_to_string(path.clone()).unwrap();
        SourceFile::new(db, name, path, text, module)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_path_name() {
        let path = Path::new("src/db/input.gib");
        let name = get_path_name(path);
        assert_eq!(name, "input");
    }

    #[test]
    fn test_get_module_path() {
        let db = SourceDatabase {
            root: "src".to_string(),
            ..Default::default()
        };
        let path = Path::new("src/db/input.gib");
        let module = get_module_path(&db, path);
        assert_eq!(module, vec!["db".to_string(), "input".to_string()]);
    }

    #[test]
    fn example() {
        let db = SourceDatabase {
            root: "/home/james/projects/another-giblang-impl".to_string(),
            ..Default::default()
        };
        let path = Path::new("/home/james/projects/another-giblang-impl/gib_mod/another.gib");
        let module = get_module_path(&db, path);
        assert_eq!(module, vec!["gib_mod".to_string(), "another".to_string()]);
    }
}
