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
