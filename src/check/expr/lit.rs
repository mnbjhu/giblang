use crate::{check::state::CheckState, lexer::literal::Literal, ty::Ty, util::Span};

impl From<&Literal> for Ty {
    fn from(value: &Literal) -> Self {
        match value {
            Literal::String(_) => Ty::Named {
                name: 1,
                args: vec![],
            },
            Literal::Int(_) => Ty::Named {
                name: 2,
                args: vec![],
            },
            Literal::Bool(_) => Ty::Named {
                name: 3,
                args: vec![],
            },
            Literal::Float(_) => Ty::Named {
                name: 4,
                args: vec![],
            },
            Literal::Char(_) => Ty::Named {
                name: 5,
                args: vec![],
            },
        }
    }
}

impl Literal {
    pub fn expect_instance_of(&self, expected: &Ty, state: &mut CheckState, span: Span) {
        let actual = Ty::from(self);
        actual.expect_is_instance_of(expected, state, false, span);
    }
}
