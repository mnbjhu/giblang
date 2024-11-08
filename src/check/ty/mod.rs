use std::ops::ControlFlow;

use crate::{
    check::state::CheckState,
    item::AstItem,
    parser::common::type_::Type,
    ty::{FuncTy, Generic, Ty},
    util::Span,
};

use super::{Check, ControlIter, Dir};
pub mod named;

impl Type {
    #[must_use]
    pub fn expect_is_bound_by<'ast, 'db, Iter: ControlIter<'ast, 'db>>(
        &'ast self,
        bound: &Generic<'db>,
        state: &mut CheckState<'db>,
        span: Span,
        control: &mut Iter,
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), Ty<'db>> {
        let ty = self.check(state, control, span, ())?;
        if let Ty::TypeVar { id } = ty {
            state.type_state.add_bound(id, bound.clone());
        } else {
            ty.expect_is_instance_of(&bound.super_, state, span);
        }
        ControlFlow::Continue(ty)
    }
}

impl<'ast, 'db, Iter: ControlIter<'ast, 'db>> Check<'ast, 'db, Iter> for Type {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: Span,
        (): (),
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), Ty<'db>> {
        control.act(self, state, Dir::Enter, span)?;
        let ty = match &self {
            Type::Named(named) => named.check(state, control, span, ())?,
            Type::Tuple(tup) => {
                let mut tys = vec![];
                for (ty, _) in tup {
                    tys.push(ty.check(state, control, span, ())?);
                }
                Ty::Tuple(tys)
            }
            Type::Sum(tup) => {
                let mut tys = vec![];
                for (ty, _) in tup {
                    tys.push(ty.check(state, control, span, ())?);
                }
                Ty::Sum(tys)
            }
            Type::Function {
                receiver,
                args,
                ret,
            } => {
                let receiver = if let Some(receiver) = receiver {
                    Some(Box::new(receiver.0.check(
                        state,
                        control,
                        receiver.1,
                        (),
                    )?))
                } else {
                    None
                };
                let mut arg_tys = vec![];
                for (arg, span) in args {
                    arg_tys.push(arg.check(state, control, *span, ())?);
                }
                Ty::Function(FuncTy {
                    receiver,
                    args: arg_tys,
                    ret: Box::new(ret.0.check(state, control, ret.1, ())?),
                })
            }
            Type::Wildcard(s) => {
                let id = state.type_state.new_type_var(*s, state.file_data);
                Ty::TypeVar { id }
            }
        };
        control.act(self, state, Dir::Exit(ty.clone()), span)?;
        ControlFlow::Continue(ty)
    }
}
