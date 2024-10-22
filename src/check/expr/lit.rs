use crate::{
    check::state::CheckState,
    db::{input::Db, modules::ModulePath},
    lexer::literal::Literal,
    ty::Ty,
    util::Span,
};

impl Literal {
    pub fn expect_instance_of<'db>(
        &self,
        expected: &Ty<'db>,
        state: &mut CheckState<'db>,
        span: Span,
    ) {
        let actual = self.to_ty(state.db);
        actual.expect_is_instance_of(expected, state, false, span);
    }

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
