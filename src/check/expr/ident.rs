use std::ops::ControlFlow;

use crate::{
    check::{
        err::{unresolved::Unresolved, CheckError},
        state::CheckState,
        Check, ControlIter, Dir,
    },
    db::decl::DeclKind,
    item::AstItem,
    parser::expr::qualified_name::SpannedQualifiedName,
    ty::Ty,
    util::Span,
};

impl<'ast, 'db, Iter: ControlIter<'ast, 'db>> Check<'ast, 'db, Iter> for SpannedQualifiedName {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: Span,
        (): (),
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), Ty<'db>> {
        control.act(self, state, Dir::Enter, span)?;
        let name = self.last().unwrap();
        if self.len() == 1 {
            if let Some(self_param) = state.get_variable("self") {
                if let Some(field) = self_param
                    .ty
                    .fields(state)
                    .iter()
                    .find(|(n, _)| n == &name.0)
                {
                    let ty = field.1.clone();
                    control.act(self, state, Dir::Exit(ty.clone()), span)?;
                    return ControlFlow::Continue(ty);
                }
                if let Some(func) = self_param
                    .ty
                    .member_funcs(state, name.1)
                    .iter()
                    .find(|(n, _)| n == &name.0)
                {
                    let ty = Ty::Function(func.1.clone());
                    control.act(self, state, Dir::Exit(ty.clone()), span)?;
                    return ControlFlow::Continue(ty);
                }
            }
            if let Some(var) = state.get_variable(&name.0) {
                control.act(self, state, Dir::Exit(var.ty.clone()), span)?;
                return ControlFlow::Continue(var.ty);
            } else if let Some(generic) = state.get_generic(&self[0].0).cloned() {
                let ty = Ty::Meta(Box::new(Ty::Generic(generic)));
                control.act(self, state, Dir::Exit(ty.clone()), span)?;
                return ControlFlow::Continue(ty);
            }
            match state.get_decl_with_error(self) {
                Ok(found) => {
                    let ty = found.get_ty(state).inst(state, name.1);
                    control.act(self, state, Dir::Exit(ty.clone()), span)?;
                    return ControlFlow::Continue(ty);
                }
                Err(e) => {
                    state.error(CheckError::Unresolved(e));
                }
            }

            control.act(self, state, Dir::Exit(Ty::Unknown), span)?;
            return ControlFlow::Continue(Ty::Unknown);
        }

        let parent = &self[..self.len() - 1];
        if let Ok(parent_decl) = state.get_decl_with_error(parent) {
            if let Some(export) = parent_decl.get(state.db, &name.0) {
                let ty = export.get_ty(state).inst(state, self.last().unwrap().1);
                control.act(self, state, Dir::Exit(ty.clone()), span)?;
                return ControlFlow::Continue(ty);
            }
            if let DeclKind::Trait { .. } | DeclKind::Enum { .. } | DeclKind::Struct { .. } =
                parent_decl.kind(state.db)
            {
                let funcs = parent_decl
                    .static_funcs(state, name.1)
                    .iter()
                    .filter_map(|(name, ty)| {
                        if name == &self.last().unwrap().0 {
                            Some(Ty::Function(ty.clone()))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                match funcs.len() {
                    0 => {}
                    1 => {
                        let ty = funcs[0].clone();
                        control.act(self, state, Dir::Exit(ty.clone()), span)?;
                        return ControlFlow::Continue(ty);
                    }
                    _ => {
                        state.simple_error("Ambiguous function", self.last().unwrap().1);
                        control.act(self, state, Dir::Exit(Ty::Unknown), span)?;
                        return ControlFlow::Continue(Ty::Unknown);
                    }
                }
            }
            state.error(CheckError::Unresolved(Unresolved {
                name: self.last().unwrap().clone(),
                file: state.file_data,
            }));
        }
        control.act(self, state, Dir::Exit(Ty::Unknown), span)?;
        ControlFlow::Continue(Ty::Unknown)
    }

    fn expect(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        expected: &Ty<'db>,
        span: Span,
        args: (),
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), Ty<'db>> {
        let actual = self.check(state, control, span, args)?;
        actual.expect_is_instance_of(expected, state, false, span);
        ControlFlow::Continue(actual)
    }
}
