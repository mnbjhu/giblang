use std::collections::HashMap;

use func::Function;
use impl_::ImplForDecl;
use salsa::Update;
use struct_::StructDecl;

pub mod func;
pub mod impl_;
pub mod struct_;

use crate::{
    check::{
        err::unresolved::Unresolved, is_scoped::IsScoped, scoped_state::Scoped, state::CheckState,
        TokenKind,
    },
    item::definitions::ident::IdentDef,
    ty::{sub_tys::get_sub_tys, FuncTy, Generic, Named, Ty},
    util::{Span, Spanned},
};

use super::{
    input::{Db, SourceFile, Vfs, VfsInner},
    path::ModulePath,
};

#[salsa::tracked]
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

impl<'db> Vfs {
    pub fn source_files(self, db: &'db dyn Db) -> Vec<&SourceFile> {
        match self.inner(db) {
            VfsInner::File(f) => vec![f],
            VfsInner::Dir(dir) => dir.iter().flat_map(|m| m.source_files(db)).collect(),
        }
    }
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

    pub fn get_ty(self, state: &impl IsScoped<'db>) -> Ty<'db> {
        match self.kind(state.db()) {
            DeclKind::Struct {
                body: StructDecl::None,
                ..
            } => self.get_named_ty(state),
            DeclKind::Trait { .. } | DeclKind::Enum { .. } | DeclKind::Struct { .. } => {
                Ty::Meta(Box::new(self.default_named_ty(state)))
            }
            DeclKind::Member { body, .. } => {
                let self_ty = self.get_named_ty(state);
                if let StructDecl::None = body {
                    return self_ty;
                }
                if let Some(ty) = body.get_constructor_ty(self_ty) {
                    Ty::Function(ty)
                } else {
                    Ty::Unknown
                }
            }
            DeclKind::Function(f) => Ty::Function(f.get_ty()),
            DeclKind::Module(_) => Ty::unit(),
        }
    }

    pub fn default_named_ty(self, state: &impl IsScoped<'db>) -> Ty<'db> {
        Ty::Named(Named {
            name: self.path(state.db()),
            args: self
                .generics(state.db())
                .iter()
                .cloned()
                .map(Ty::Generic)
                .collect(),
        })
    }

    pub fn get_named_ty(self, state: &impl IsScoped<'db>) -> Ty<'db> {
        if let DeclKind::Member { .. } = &self.kind(state.db()) {
            let parent = self.path(state.db()).get_parent(state.db());
            let parent_decl = state.try_get_decl_path(parent);
            parent_decl
                .expect("No parent found for decl")
                .default_named_ty(state)
        } else {
            self.default_named_ty(state)
        }
    }

    pub fn static_funcs(
        self,
        state: &mut CheckState<'db>,
        span: Span,
    ) -> Vec<(Decl<'db>, FuncTy<'db>)> {
        if !matches!(
            self.kind(state.db),
            DeclKind::Trait { .. } | DeclKind::Enum { .. } | DeclKind::Struct { .. }
        ) {
            return Vec::new();
        }
        let ty = self.default_named_ty(state).inst(state, span);
        let mut funcs = get_sub_tys(&ty, state)
            .iter()
            .flat_map(|t| t.get_funcs(state))
            .collect::<Vec<_>>();
        funcs.extend(ty.get_funcs(state));
        funcs
    }
}

impl<'db> Function<'db> {
    pub fn get_ty(&self) -> FuncTy<'db> {
        let args = self
            .args
            .iter()
            .map(|(_, ty)| ty.clone())
            .collect::<Vec<_>>();
        FuncTy {
            receiver: self.receiver.clone().map(Box::new),
            args,
            ret: Box::new(self.ret.clone()),
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

    pub fn get_path_ir(
        self,
        state: &impl Scoped<'db>,
        path: &[Spanned<String>],
    ) -> Vec<Spanned<IdentDef<'db>>> {
        let mut current = self;
        let mut found = vec![];
        for name in path {
            if let Some(decl) = current.get(state.db(), &name.0) {
                found.push((IdentDef::Decl(decl), name.1));
                current = decl;
            } else {
                found.push((IdentDef::Unresolved, name.1));
                return found;
            }
        }
        found
    }

    pub fn try_get_path(
        self,
        state: &impl Scoped<'db>,
        path: &[Spanned<String>],
    ) -> Result<Decl<'db>, Unresolved> {
        let mut current = self;
        for name in path {
            if let Some(decl) = current.get(state.db(), &name.0) {
                current = decl;
            } else {
                return Err(Unresolved {
                    name: (name.0.clone(), name.1),
                    file: state.get_file(),
                });
            }
        }
        Ok(current)
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
