use salsa::Update;

use crate::{
    check::{state::CheckState, TokenKind},
    db::{
        input::{Db, SourceFile},
        modules::{Module, ModulePath},
    },
    ty::{prim::PrimTy, FuncTy, Generic, Ty},
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
    pub file: SourceFile,
    pub path: ModulePath<'db>,
}

impl<'db> Decl<'db> {
    pub fn get_kind(&self, db: &'db dyn Db) -> TokenKind {
        match &self.kind(db) {
            DeclKind::Trait { .. } => TokenKind::Trait,
            DeclKind::Enum { .. } => TokenKind::Enum,
            DeclKind::Function { .. } => TokenKind::Func,
            DeclKind::Member { .. } => TokenKind::Member,
            DeclKind::Prim(_) | DeclKind::Struct { .. } => TokenKind::Struct,
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
        body: Vec<Module<'db>>,
    },
    Enum {
        generics: Vec<Generic<'db>>,
        variants: Vec<Module<'db>>,
    },
    Member {
        body: StructDecl<'db>,
    },
    Function {
        generics: Vec<Generic<'db>>,
        receiver: Option<Ty<'db>>,
        args: Vec<(String, Ty<'db>)>,
        ret: Ty<'db>,
    },
    Prim(PrimTy),
}

#[salsa::tracked]
impl<'db> Decl<'db> {
    #[must_use]
    #[salsa::tracked]
    pub fn generics(self, db: &'db dyn Db) -> Vec<Generic<'db>> {
        match self.kind(db) {
            DeclKind::Struct { generics, .. }
            | DeclKind::Trait { generics, .. }
            | DeclKind::Enum { generics, .. }
            | DeclKind::Function { generics, .. } => generics.clone(),
            DeclKind::Member { .. } => {
                panic!("Hmm, don't think I need this, guess I'll find out")
            }
            DeclKind::Prim(_) => vec![],
        }
    }

    #[must_use]
    #[salsa::tracked]
    pub fn get(self, db: &'db dyn Db, name: String) -> Option<Module<'db>> {
        match self.kind(db) {
            DeclKind::Enum { variants, .. } => {
                variants.iter().find(|v| v.name(db) == name).copied()
            }
            DeclKind::Trait { body, .. } => body.iter().find(|m| m.name(db) == name).copied(),
            _ => None,
        }
    }

    pub fn get_ty(&self, id: ModulePath<'db>, state: &mut CheckState<'_, 'db>) -> Ty<'db> {
        match self.kind(state.db) {
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
            DeclKind::Function {
                receiver,
                args,
                ret,
                ..
            } => {
                let args = args.iter().map(|(_, ty)| ty.clone()).collect::<Vec<_>>();
                Ty::Function(FuncTy {
                    receiver: receiver.clone().map(Box::new),
                    args,
                    ret: Box::new(ret.clone()),
                })
            }
            DeclKind::Prim(p) => Ty::Meta(Box::new(Ty::from_prim(*p, state.db))),
        }
    }

    fn default_named_ty(self, state: &mut CheckState<'_, 'db>, name: ModulePath<'db>) -> Ty<'db> {
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

    pub fn get_named_ty(self, state: &mut CheckState<'_, 'db>, id: ModulePath<'db>) -> Ty<'db> {
        if let DeclKind::Member { .. } = &self.kind(state.db) {
            let parent = id.get_parent(state.db);
            let parent_decl = state.project.get_decl(state.db, parent);
            if parent_decl.is_none() {
                return Ty::Unknown;
            }
            parent_decl.unwrap().default_named_ty(state, parent)
        } else {
            self.default_named_ty(state, id)
        }
    }
}
