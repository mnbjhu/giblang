use salsa::{Accumulator, Update};
use tracing::info;

use crate::{
    check::{
        err::{unresolved::Unresolved, IntoWithDb},
        state::CheckState,
    },
    parser::expr::qualified_name::SpannedQualifiedName,
    project::decl::Decl,
    util::Spanned,
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
    pub fn get(self, db: &'db dyn Db, name: String) -> Option<Module<'db>> {
        info!("'get' for {:?}", name);
        match self.content(db) {
            ModuleData::Package(modules) => modules.iter().find(|m| m.name(db) == name).copied(),
            ModuleData::Export(export) => export.get(db, name),
        }
    }

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

    pub fn get_path_with_state(
        self,
        state: &mut CheckState<'_, 'db>,
        path: &[Spanned<String>],
        file: SourceFile,
        should_error: bool,
    ) -> Option<Module<'db>> {
        self.get_path_with_state_inner(state, path, file, &mut Vec::new(), should_error)
    }

    fn get_path_with_state_inner(
        self,
        state: &mut CheckState<'_, 'db>,
        path: &[Spanned<String>],
        file: SourceFile,
        current: &mut Vec<String>,
        should_error: bool,
    ) -> Option<Module<'db>> {
        if path.is_empty() {
            Some(self)
        } else if let Some(found) = self.get(state.db, path[0].0.to_string()) {
            current.push(path[0].0.to_string());
            let id = ModulePath::new(state.db, current.clone());
            found.get_path_with_state_inner(state, &path[1..], file, current, should_error)
        } else {
            if should_error {
                Unresolved {
                    name: (path[0].0.to_string(), path[0].1),
                    file,
                }
                .into_with_db(state.db)
                .accumulate(state.db);
            }
            None
        }
    }
}
