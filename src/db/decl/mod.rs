use std::collections::HashMap;

use func::Function;
use impl_::ImplForDecl;
use salsa::{Accumulator, Update};
use struct_::StructDecl;

pub mod impl_;
pub mod struct_;
pub mod func;

use crate::{
    check::{
        err::{unresolved::Unresolved, IntoWithDb},
        state::CheckState, TokenKind,
    }, parser::expr::qualified_name::SpannedQualifiedName, ty::{FuncTy, Generic, Ty}, util::{Span, Spanned}
};

use super::{
    input::{Db, SourceFile},
    path::ModulePath,
};


#[salsa::tracked]
#[derive()]
pub struct Project<'db> {
    pub decls: Decl<'db>,
    pub impl_map: HashMap<ModulePath<'db>, Vec<ImplForDecl<'db>>>,
}


impl<'db> Project<'db> {
    pub fn get_decl(self, db: &'db dyn Db, path: ModulePath<'db>) -> Option<Decl<'db>> {
        self.decls(db).get_path(db, path)
    }

    pub fn get_impls(self, db: &'db dyn Db, path: ModulePath<'db>) -> Vec<ImplForDecl<'db>> {
        self.impl_map(db).get(&path).cloned().unwrap_or_default()
    }
}
#[salsa::tracked]
pub struct Decl<'db> {
    #[id]
    pub name: String,
    pub span: Span,
    #[return_ref]
    pub kind: DeclKind<'db>,
    pub maybe_file: Option<SourceFile>,
    pub path: ModulePath<'db>,
}


#[derive(Update, Debug, Clone, PartialEq)]
pub enum DeclKind<'db> {
    Struct {
        generics: Vec<Generic<'db>>,
        body: StructDecl<'db>,
    },
    Trait {
        generics: Vec<Generic<'db>>,
        body: Vec<Decl<'db>>,
    },
    Enum {
        generics: Vec<Generic<'db>>,
        variants: Vec<Decl<'db>>,
    },
    Member {
        body: StructDecl<'db>,
    },
    Function(Function<'db>),
    Module(Vec<Decl<'db>>),
}

#[salsa::tracked]
impl<'db> Decl<'db> {
    #[must_use]
    pub fn generics(self, db: &'db dyn Db) -> Vec<Generic<'db>> {
        match self.kind(db) {
            DeclKind::Struct { generics, .. }
            | DeclKind::Trait { generics, .. }
            | DeclKind::Enum { generics, .. }
            | DeclKind::Function(Function { generics, .. }) => generics.clone(),
            DeclKind::Member { .. } | DeclKind::Module(_) => {
                panic!("Generics not supported for this decl kind")
            }
        }
    }

    pub fn get_ty(self, state: &CheckState<'db>) -> Ty<'db> {
        let id = self.path(state.db);
        match self.kind(state.db) {
            DeclKind::Struct {
                body: StructDecl::None,
                ..
            } => self.get_named_ty(state, id),
            DeclKind::Trait { .. } | DeclKind::Enum { .. } | DeclKind::Struct { .. } => {
                Ty::Meta(Box::new(self.default_named_ty(state, id)))
            }
            DeclKind::Member { body, .. } => {
                let self_ty = self.get_named_ty(state, id);
                if let StructDecl::None = body {
                    return self_ty;
                }
                if let Some(ty) = body.get_constructor_ty(self_ty) {
                    Ty::Function(ty)
                } else {
                    Ty::Unknown
                }
            }
            DeclKind::Function(Function {
                receiver,
                args,
                ret,
                ..
            }) => {
                let args = args.iter().map(|(_, ty)| ty.clone()).collect::<Vec<_>>();
                Ty::Function(FuncTy {
                    receiver: receiver.clone().map(Box::new),
                    args,
                    ret: Box::new(ret.clone()),
                })
            }
            DeclKind::Module(_) => Ty::unit(),
        }
    }

    fn default_named_ty(self, state: &CheckState<'db>, name: ModulePath<'db>) -> Ty<'db> {
        Ty::Named {
            name,
            args: self
                .generics(state.db)
                .iter()
                .cloned()
                .map(Ty::Generic)
                .collect(),
        }
    }

    pub fn get_named_ty(self, state: &CheckState<'db>, id: ModulePath<'db>) -> Ty<'db> {
        if let DeclKind::Member { .. } = &self.kind(state.db) {
            let parent = id.get_parent(state.db);
            let parent_decl = state.try_get_decl(parent);
            if parent_decl.is_none() {
                return Ty::Unknown;
            }
            parent_decl.unwrap().default_named_ty(state, parent)
        } else {
            self.default_named_ty(state, id)
        }
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

    pub fn into_func(self, db: &'db dyn Db) -> &'db Function<'db> {
        if let DeclKind::Function(f) = self.kind(db) {
            f
        } else {
            panic!("Expected function");
        }
    }

    pub fn file(self, db: &'db dyn Db) -> SourceFile {
        self.maybe_file(db).expect("Modules do not have files")
    }
    pub fn get_kind(self, db: &'db dyn Db) -> TokenKind {
        match &self.kind(db) {
            DeclKind::Trait { .. } => TokenKind::Trait,
            DeclKind::Enum { .. } => TokenKind::Enum,
            DeclKind::Function(Function { .. }) => TokenKind::Func,
            DeclKind::Member { .. } => TokenKind::Member,
            DeclKind::Struct { .. } => TokenKind::Struct,
            DeclKind::Module(_) => TokenKind::Module,
        }
    }
}
