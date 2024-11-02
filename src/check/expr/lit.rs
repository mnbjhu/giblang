use std::ops::ControlFlow;

use crate::{
    check::{state::CheckState, Check, ControlIter, Dir},
    db::{input::Db, path::ModulePath},
    item::AstItem,
    lexer::literal::Literal,
    ty::Ty,
    util::Span,
};

impl Literal {
    pub fn to_ty<'db>(&self, db: &'db dyn Db) -> Ty<'db> {
        match self {
            Literal::String(_) => Ty::Named {
                name: ModulePath::new(db, vec!["std".to_string(), "String".to_string()]),
                args: vec![],
            },
            Literal::Int(_) => Ty::Named {
                name: ModulePath::new(db, vec!["std".to_string(), "Int".to_string()]),
                args: vec![],
            },
            Literal::Bool(_) => Ty::Named {
                name: ModulePath::new(db, vec!["std".to_string(), "Bool".to_string()]),
                args: vec![],
            },
            Literal::Float(_) => Ty::Named {
                name: ModulePath::new(db, vec!["std".to_string(), "Float".to_string()]),
                args: vec![],
            },
            Literal::Char(_) => Ty::Named {
                name: ModulePath::new(db, vec!["std".to_string(), "Char".to_string()]),
                args: vec![],
            },
        }
    }
}

impl<'ast, 'db, Iter: ControlIter<'ast, 'db>> Check<'ast, 'db, Iter> for Literal {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: Span,
        (): (),
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), Ty<'db>> {
        control.act(self, state, Dir::Enter, span);
        let actual = self.to_ty(state.db);
        control.act(self, state, Dir::Exit(actual.clone()), span);
        ControlFlow::Continue(actual)
    }

    fn expect(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        expected: &Ty<'db>,
        span: Span,
        (): (),
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), Ty<'db>> {
        control.act(self, state, Dir::Enter, span);
        let actual = self.to_ty(state.db);
        actual.expect_is_instance_of(expected, state, false, span);
        control.act(self, state, Dir::Exit(actual.clone()), span);
        ControlFlow::Continue(actual)
    }
}
