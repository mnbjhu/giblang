use salsa::Accumulator;

use crate::{
    check::{
        err::{unresolved::Unresolved, IntoWithDb},
        state::CheckState,
    },
    parser::expr::qualified_name::SpannedQualifiedName,
    project::decl::{Decl, DeclKind},
    util::Spanned,
};

use super::input::{Db, SourceFile};

#[salsa::interned]
pub struct ModulePath<'db> {
    #[return_ref]
    pub name: Vec<String>,
}

impl<'db> ModulePath<'db> {
    #[must_use]
    pub fn get_parent(self, db: &'db dyn Db) -> ModulePath<'db> {
        let path = self.name(db);
        ModulePath::new(db, path[0..path.len() - 1].to_vec())
    }
}

#[salsa::tracked]
impl<'db> Decl<'db> {
    pub fn get(self, db: &'db dyn Db, name: &str) -> Option<Decl<'db>> {
        match self.kind(db) {
            DeclKind::Module(modules) => modules.iter().find(|m| m.name(db) == name).copied(),
            DeclKind::Enum { variants, .. } => {
                variants.iter().find(|v| v.name(db) == name).copied()
            }
            DeclKind::Trait { body, .. } => body.iter().find(|m| m.name(db) == name).copied(),
            _ => None,
        }
    }

    #[salsa::tracked]
    pub fn get_path(self, db: &'db dyn Db, path: ModulePath<'db>) -> Option<Decl<'db>> {
        let mut current = self;
        for name in path.name(db) {
            current = current.get(db, name)?;
        }
        Some(current)
    }

    #[salsa::tracked]
    pub fn get_path_without_error(
        self,
        db: &'db dyn Db,
        path: SpannedQualifiedName,
    ) -> Option<Decl<'db>> {
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
    ) -> Option<Decl<'db>> {
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
        state: &mut CheckState<'db>,
        path: &[Spanned<String>],
        file: SourceFile,
        should_error: bool,
    ) -> Option<Decl<'db>> {
        self.get_path_with_state_inner(state, path, file, &mut Vec::new(), should_error)
    }

    fn get_path_with_state_inner(
        self,
        state: &mut CheckState<'db>,
        path: &[Spanned<String>],
        file: SourceFile,
        current: &mut Vec<String>,
        should_error: bool,
    ) -> Option<Decl<'db>> {
        if path.is_empty() {
            Some(self)
        } else if let Some(found) = self.get(state.db, &path[0].0) {
            current.push(path[0].0.to_string());
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
