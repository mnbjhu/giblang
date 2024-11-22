use std::{collections::HashMap, ops::ControlFlow};

use crate::{
    check::{state::CheckState, Check, ControlIter, Dir},
    db::decl::{struct_::StructDecl, DeclKind},
    item::AstItem,
    parser::expr::field::Field,
    ty::{Named, Ty},
    util::Span,
};

impl<'ast, 'db, Iter: ControlIter<'ast, 'db>> Check<'ast, 'db, Iter> for Field {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: Span,
        (): (),
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), Ty<'db>> {
        control.act(self, state, Dir::Enter, span)?;
        let struct_ty = self.struct_.0.check(state, control, self.struct_.1, ())?;
        if self.name.0.is_empty() {
            return ControlFlow::Continue(Ty::Unknown);
        }
        if let Ty::Named(Named { name, args }) = struct_ty {
            let decl = state.try_get_decl_path(name);
            if let Some(decl) = decl {
                if let DeclKind::Struct { body, generics } = decl.kind(state.db) {
                    let params = generics
                        .iter()
                        .map(|arg| arg.name.0.clone())
                        .zip(args.iter().cloned())
                        .collect::<HashMap<_, _>>();
                    match body {
                        StructDecl::Fields(fields) => {
                            if let Some(field) = fields.iter().find(|field| field.0 == self.name.0)
                            {
                                let ty = field.1.parameterize(&params);
                                control.act(self, state, Dir::Exit(ty.clone()), span)?;
                                return ControlFlow::Continue(ty);
                            }
                            state.simple_error(
                                &format!(
                                    "No field {} found on struct {}",
                                    self.name.0,
                                    name.name(state.db).join("::")
                                ),
                                self.name.1,
                            );
                        }
                        StructDecl::Tuple(tys) => {
                            if let Ok(index) = self.name.0.parse::<usize>() {
                                if index >= tys.len() {
                                    state.simple_error(
                                        &format!("Index out of bounds: {} >= {}", index, tys.len()),
                                        self.name.1,
                                    );
                                }
                                let ty = tys[index].parameterize(&params);
                                control.act(self, state, Dir::Exit(ty.clone()), span)?;
                                return ControlFlow::Continue(ty);
                            }
                            state.simple_error("Expected integer index", self.name.1);
                        }
                        StructDecl::None => {
                            state.simple_error("A unit struct has no fields", self.name.1);
                        }
                    }
                } else {
                    state.simple_error(
                        &format!(
                            "Expected struct but found {}",
                            decl.path(state.db).name(state.db).join("::")
                        ),
                        self.struct_.1,
                    );
                }
            }
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
        (): (),
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), Ty<'db>> {
        let ty: Ty<'db> = self.check(state, control, span, ())?;
        ty.expect_is_instance_of(expected, state, span);
        ControlFlow::Continue(ty)
    }
}
