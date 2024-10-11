use salsa::{Accumulator, Update};

use crate::{
    check::err::{unresolved::Unresolved, CheckError, Error},
    parser::expr::qualified_name::SpannedQualifiedName,
    project::decl::Decl,
};

use super::input::{Db, SourceFile};

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
    pub fn get_parent(self, db: &'db dyn Db) -> ModulePath<'db> {
        let path = self.name(db);
        ModulePath::new(db, path[0..path.len() - 1].to_vec())
    }
}

#[salsa::tracked]
impl<'db> Module<'db> {
    #[salsa::tracked]
    pub fn get_path(self, db: &'db dyn Db, path: ModulePath<'db>) -> Option<Module<'db>> {
        let mut current = self;
        for name in path.name(db) {
            match current.content(db) {
                ModuleData::Package(modules) => {
                    current = modules.iter().find(|m| m.name(db) == *name).copied()?;
                }
                ModuleData::Export(_) => return None,
            }
        }
        Some(current)
    }

    #[salsa::tracked]
    pub fn get_path_without_error(
        self,
        db: &'db dyn Db,
        path: SpannedQualifiedName,
    ) -> Option<Module<'db>> {
        let segs = path
            .iter()
            .map(|(name, _)| name.to_string())
            .collect::<Vec<_>>();
        let module_path = ModulePath::new(db, segs.clone());
        self.get_path(db, module_path)
    }

    #[salsa::tracked]
    pub fn get_path_with_error(
        self,
        db: &'db dyn Db,
        path: SpannedQualifiedName,
        file: SourceFile,
    ) -> Option<Module<'db>> {
        let span = path.first().unwrap().1.start..path.last().unwrap().1.end;
        let segs = path
            .iter()
            .map(|(name, _)| name.to_string())
            .collect::<Vec<_>>();
        let module_path = ModulePath::new(db, segs.clone());
        let found = self.get_path(db, module_path);
        if found.is_none() {
            Error {
                inner: CheckError::Unresolved(Unresolved {
                    name: (segs.join("::"), span.into()),
                    file,
                }),
            }
            .accumulate(db);
        };
        found
    }
}
