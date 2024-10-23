use std::collections::HashMap;

use salsa::Update;

use crate::{
    check::{state::CheckState, TokenKind},
    db::{
        input::{Db, SourceFile},
        modules::ModulePath,
    },
    ty::{FuncTy, Generic, Ty},
    util::Span,
};

use self::struct_::StructDecl;

pub mod impl_;
pub mod struct_;

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

impl<'db> Decl<'db> {
    pub fn file(&self, db: &'db dyn Db) -> SourceFile {
        self.maybe_file(db).expect("Modules do not have files")
    }
}

impl<'db> Decl<'db> {
    pub fn get_kind(&self, db: &'db dyn Db) -> TokenKind {
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

#[derive(Update, Debug, Clone, PartialEq)]
pub struct Function<'db> {
    pub name: String,
    pub generics: Vec<Generic<'db>>,
    pub receiver: Option<Ty<'db>>,
    pub args: Vec<(String, Ty<'db>)>,
    pub ret: Ty<'db>,
    pub required: bool,
}

impl<'db> Generic<'db> {
    pub fn parameterize(&self, params: &HashMap<String, Ty<'db>>) -> Generic<'db> {
        Generic {
            name: self.name.clone(),
            variance: self.variance,
            super_: Box::new(self.super_.parameterize(params)),
        }
    }
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

    pub fn get_ty(&self, state: &CheckState<'db>) -> Ty<'db> {
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
