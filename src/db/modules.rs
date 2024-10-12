use salsa::{Accumulator, Update};
use tracing::info;

use crate::{
    check::err::{unresolved::Unresolved, IntoWithDb},
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
    #[must_use]
    pub fn get_parent(self, db: &'db dyn Db) -> ModulePath<'db> {
        info!("'get_parent' for {:?}", self.name(db));
        let path = self.name(db);
        ModulePath::new(db, path[0..path.len() - 1].to_vec())
    }
}

#[salsa::tracked]
impl<'db> Module<'db> {
    #[salsa::tracked]
    pub fn get_path(self, db: &'db dyn Db, path: ModulePath<'db>) -> Option<Module<'db>> {
        info!("'get_path' for {:?}", path);
        let mut current = self;
        for name in path.name(db) {
            info!(
                "'get_path' found {:?} with inner {:?}",
                path.name(db),
                current.content(db)
            );
            match current.content(db) {
                ModuleData::Package(modules) => {
                    current = modules.iter().find(|m| m.name(db) == *name).copied()?;
                }
                ModuleData::Export(export) => current = export.get(db, name.to_string())?,
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
        if path.is_empty() {
            return Some(self);
        }
        let span = path.first().unwrap().1.start..path.last().unwrap().1.end;
        let segs = path
            .iter()
            .map(|(name, _)| name.to_string())
            .collect::<Vec<_>>();
        let module_path = ModulePath::new(db, segs.clone());
        let found = self.get_path(db, module_path);
        if found.is_none() {
            Unresolved {
                name: (segs.join("::"), span.into()),
                file,
            }
            .into_with_db(db)
            .accumulate(db);
        };
        found
    }
}
