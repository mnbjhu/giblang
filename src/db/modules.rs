use salsa::{Database, Update};

use crate::project::decl::Decl;

#[salsa::tracked]
pub struct Module<'db> {
    #[id]
    pub name: String,

    #[return_ref]
    pub content: ModuleData<'db>,
}

#[derive(Update, Clone, Debug)]
pub enum ModuleData<'db> {
    Package(Vec<Module<'db>>),
    Export(Decl<'db>),
}

#[salsa::interned]
pub struct ModulePath<'db> {
    #[return_ref]
    pub name: Vec<String>,
}

impl<'db> ModulePath<'db> {
    pub fn get_parent(self, db: &'db dyn Database) -> ModulePath<'db> {
        let path = self.name(db);
        ModulePath::new(db, path[0..path.len() - 1].to_vec())
    }
}

#[salsa::tracked]
impl<'db> Module<'db> {
    #[salsa::tracked]
    pub fn get_path(self, db: &'db dyn Database, path: ModulePath<'db>) -> Option<Module<'db>> {
        let mut current = self;
        for name in path.name(db) {
            match current.content(db) {
                ModuleData::Package(modules) => {
                    current = modules.iter().find(|m| m.name(db) == *name).cloned()?;
                }
                ModuleData::Export(_) => return None,
            }
        }
        Some(current)
    }
}
