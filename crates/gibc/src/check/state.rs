use salsa::{Accumulator, Update};
use std::{collections::HashMap, vec};

use crate::{
    check::err::{simple::Simple, CheckError},
    db::{
        decl::{Decl, Project},
        input::{Db, SourceFile},
    },
    parser::{common::variance::Variance, expr::qualified_name::SpannedQualifiedName, parse_file},
    ty::{Generic, Ty},
    util::Span,
};

use super::{
    err::{unresolved::Unresolved, unresolved_type_var::UnboundTypeVar, Error, IntoWithDb},
    is_scoped::IsScoped,
    scoped_state::{Scoped, ScopedState},
    type_state::TypeState,
    TokenKind,
};

#[derive(Debug, PartialEq, Clone, Update, Eq)]
pub struct VarDecl<'db> {
    pub name: String,
    pub ty: Ty<'db>,
    pub kind: TokenKind,
    pub span: Span,
}

pub struct CheckState<'db> {
    pub db: &'db dyn Db,
    pub file_data: SourceFile,
    pub project: Project<'db>,
    pub type_state: TypeState<'db>,
    pub should_error: bool,
    pub decl_stack: Vec<Decl<'db>>,
    pub scope_state: ScopedState<'db>,
}

impl<'db> IsScoped<'db> for CheckState<'db> {
    fn get_scope_state<'me>(&'me self) -> &'me ScopedState<'db> {
        &self.scope_state
    }

    fn get_scope_state_mut<'me>(&'me mut self) -> &'me mut ScopedState<'db> {
        &mut self.scope_state
    }

    fn get_type_var(&self, id: u32) -> Ty<'db> {
        self.get_resolved_type_var(id)
    }

    fn expected_type_var_is(&mut self, id: u32, other: Ty<'db>, span: Span) {
        self.expected_var_is_ty(id, other, span);
    }
}

impl<'ty, 'db: 'ty> CheckState<'db> {
    pub fn resolved_ty(&self, ty: &Ty<'db>) -> Ty<'db> {
        if let Ty::TypeVar { id } = ty {
            self.get_resolved_type_var(*id)
        } else {
            ty.clone()
        }
    }

    pub fn from_file(
        db: &'db dyn Db,
        file_data: SourceFile,
        project: Project<'db>,
    ) -> CheckState<'db> {
        let path = file_data.module_path(db);
        let decl = project.get_decl(db, path).unwrap();
        let mut state = CheckState {
            db,
            file_data,
            project,
            type_state: TypeState::default(),
            should_error: true,
            decl_stack: vec![decl],
            scope_state: ScopedState::new(db, project, file_data),
        };
        let tops = parse_file(db, file_data).tops(db);
        for top in tops {
            if let Some(name) = top.0.get_name() {
                state.add_local_import(name);
            }
        }
        state
    }

    pub fn expected_var_is_ty(&mut self, id: u32, ty: Ty<'db>, span: Span) {
        if let Ty::TypeVar { id: second } = ty {
            self.type_state.merge(id, second);
            return;
        }
        let var = self.type_state.get_type_var_mut(id);
        if let Some(resolved) = var.resolved.clone() {
            resolved.expect_is_instance_of(&ty, self, span);
            return;
        }
        var.resolved = Some(ty);
    }

    pub fn get_type_vars(&mut self) -> HashMap<u32, Ty<'db>> {
        let mut res = HashMap::new();
        for id in self.type_state.vars.keys().copied().collect::<Vec<_>>() {
            let ty = self.get_resolved_type_var(id);
            let data = self.type_state.get_type_var(id).clone();
            if let Ty::Unknown = ty {
                self.error(CheckError::UnboundTypeVar(UnboundTypeVar {
                    span: data.span,
                    file: data.file,
                }));
            }
            res.insert(id, ty);
        }
        res
    }

    pub fn add_local_import(&mut self, name: &str) {
        let decl = self.local_decl(name);
        self.insert_import(name, decl);
    }

    pub fn local_decl(&self, name: &str) -> Decl<'db> {
        self.current_decl().get(self.db, name).unwrap_or_else(|| {
            panic!(
                "Local decl not found: {} in {:#?}",
                name,
                self.current_decl()
            )
        })
    }

    pub fn current_decl(&self) -> Decl<'db> {
        self.decl_stack.last().copied().unwrap()
    }

    pub fn file_decl(&self) -> Decl<'db> {
        self.decl_stack.first().copied().unwrap()
    }

    pub fn simple_error(&mut self, message: &str, span: Span) {
        self.error(CheckError::Simple(Simple {
            message: message.to_string(),
            span,
            file: self.file_data,
        }));
    }

    pub fn error(&mut self, error: CheckError) {
        if self.should_error
            && self
                .file_decl()
                .path(self.db)
                .name(self.db)
                .first()
                .unwrap()
                != "std"
        {
            Error { inner: error }
                .into_with_db(self.db)
                .accumulate(self.db);
        }
    }

    pub fn add_self_ty(&mut self, super_: &Ty<'db>, span: Span) {
        let generic = Generic {
            name: ("Self".to_string(), span),
            variance: Variance::Invariant,
            super_: Box::new(super_.clone()),
        };
        self.insert_generic("Self", generic);
    }

    pub fn add_self_param(&mut self, ty: &Ty<'db>, span: Span) {
        let ty = if let Ty::Generic(g) = &ty {
            if g.name.0 == "Self" {
                g.super_.as_ref()
            } else {
                ty
            }
        } else {
            ty
        };
        self.insert_variable(
            "self",
            VarDecl {
                name: "self".to_string(),
                ty: ty.clone(),
                kind: TokenKind::Param,
                span,
            },
        );
    }

    pub fn import(&mut self, use_: &SpannedQualifiedName) -> Result<(), Unresolved> {
        match self.get_decl_with_error(use_) {
            Err(e) => Err(e),
            Ok(decl) => {
                self.insert_import(&use_.last().unwrap().0, decl);
                Ok(())
            }
        }
    }

    pub fn get_resolved_type_var(&self, id: u32) -> Ty<'db> {
        self.type_state
            .get_type_var(id)
            .resolved
            .clone()
            .unwrap_or(Ty::Unknown)
    }

    pub fn enter_decl(&mut self, name: &str) {
        self.decl_stack.push(self.local_decl(name));
    }

    pub fn exit_decl(&mut self) {
        self.decl_stack.pop();
    }
}
