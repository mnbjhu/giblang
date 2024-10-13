use salsa::{Database, Update};

use crate::{
    check::{state::CheckState, TokenKind},
    db::{
        input::Db,
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
}

impl<'db> Decl<'db> {
    pub fn get_kind(&self, db: &'db dyn Db) -> TokenKind {
        match &self.kind(db) {
            DeclKind::Struct { .. } => TokenKind::Struct,
            DeclKind::Trait { .. } => TokenKind::Trait,
            DeclKind::Enum { .. } => TokenKind::Enum,
            DeclKind::Function { .. } => TokenKind::Func,
            DeclKind::Member { .. } => TokenKind::Member,
            DeclKind::Prim(_) => TokenKind::Struct,
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
            DeclKind::Trait { .. } | DeclKind::Enum { .. } => Ty::Unknown,
            DeclKind::Member { body, .. } | DeclKind::Struct { body, .. } => {
                let self_ty = self.get_named_ty(state, id);
                body.get_constructor_ty(self_ty)
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

    fn get_named_ty(self, state: &mut CheckState<'_, 'db>, id: ModulePath<'db>) -> Ty<'db> {
        if let DeclKind::Member { .. } = &self.kind(state.db) {
            let parent = id.get_parent(state.db);
            let parent_decl = state.project.get_decl(state.db, parent);
            if parent_decl.is_none() {
                return Ty::Unknown;
            }
            let parent_decl = parent_decl.unwrap();
            Ty::Named {
                name: parent,
                args: parent_decl
                    .generics(state.db)
                    .iter()
                    .cloned()
                    .map(Ty::Generic)
                    .collect(),
            }
        } else {
            Ty::Named {
                name: id,
                args: self
                    .generics(state.db)
                    .iter()
                    .cloned()
                    .map(Ty::Generic)
                    .collect(),
            }
        }
    }
}
