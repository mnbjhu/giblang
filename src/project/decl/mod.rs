use salsa::{Database, Update};

use crate::{
    check::state::CheckState,
    db::modules::ModulePath,
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
    pub fn generics(self, db: &'db dyn Database) -> Vec<Generic<'db>> {
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

    pub fn get_ty(
        &'db self,
        db: &'db dyn Database,
        id: ModulePath<'db>,
        state: &mut CheckState<'_, 'db>,
    ) -> Ty<'db> {
        match self.kind(db) {
            DeclKind::Trait { .. } => todo!(),
            DeclKind::Enum { .. } => todo!(),
            DeclKind::Member { body, .. } | DeclKind::Struct { body, .. } => {
                let self_ty = self.get_named_ty(db, state, id);
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
            DeclKind::Prim(p) => Ty::Meta(Box::new(Ty::from_prim(*p, db))),
        }
    }

    fn get_named_ty(
        &self,
        db: &'db dyn Database,
        state: &mut CheckState<'_, 'db>,
        id: ModulePath<'db>,
    ) -> Ty {
        if let DeclKind::Member { .. } = &self.kind(db) {
            let parent = id.get_parent(db);
            let parent_decl = state.project.get_decl(db, id);
            Ty::Named {
                name: parent,
                args: parent_decl
                    .generics(db)
                    .iter()
                    .cloned()
                    .map(Ty::Generic)
                    .collect(),
            }
        } else {
            Ty::Named {
                name: id,
                args: self.generics(db).iter().cloned().map(Ty::Generic).collect(),
            }
        }
    }
}
