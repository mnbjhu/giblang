use std::ops::ControlFlow;

use crate::{
    check::{state::CheckState, Check, ControlIter, Dir},
    item::AstItem,
    parser::common::type_::NamedType,
    ty::Ty,
    util::Span,
};

impl<'ast, 'db, Iter: ControlIter<'ast, 'db>> Check<'ast, 'db, Iter> for NamedType {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        _: Span,
        (): (),
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), Ty<'db>> {
        let name_span = Span::new(self.name[0].1.start, self.name[self.name.len() - 1].1.end);
        control.act(&self.name, state, Dir::Enter, name_span)?;
        if self.name.len() == 1 {
            if self.name[0].0 == "Any" {
                control.act(&self.name, state, Dir::Exit(Ty::Any), name_span)?;
                return ControlFlow::Continue(Ty::Any);
            }
            if self.name[0].0 == "Nothing" {
                control.act(&self.name, state, Dir::Exit(Ty::Nothing), name_span)?;
                return ControlFlow::Continue(Ty::Nothing);
            }
            if let Some(generic) = state.get_generic(&self.name[0].0).cloned() {
                let ty = Ty::Generic(generic);
                control.act(&self.name, state, Dir::Exit(ty.clone()), name_span)?;
                return ControlFlow::Continue(ty);
            }
        };
        if let Ok(decl) = state.get_decl_with_error(&self.name) {
            let mut args = vec![];
            for (arg, gen) in self.args.iter().zip(decl.generics(state.db)) {
                args.push(arg.0.expect_is_bound_by(&gen, state, arg.1, control)?);
            }
            let ty = Ty::Named {
                name: decl.path(state.db),
                args,
            };
            control.act(&self.name, state, Dir::Exit(ty.clone()), name_span)?;
            return ControlFlow::Continue(ty);
        };
        control.act(&self.name, state, Dir::Exit(Ty::Unknown), name_span)?;
        ControlFlow::Continue(Ty::Unknown)
    }
}
