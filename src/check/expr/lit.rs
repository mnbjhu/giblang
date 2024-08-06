use crate::{
    check::state::CheckState, lexer::literal::Literal, project::Project, ty::Ty, util::Span,
};

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
    pub fn expect_instance_of(
        &self,
        expected: &Ty,
        project: &Project,
        state: &mut CheckState,
        span: Span,
    ) -> Ty {
        let actual = Ty::from(self);
        if !actual.is_instance_of(expected, project) {
            state.simple_error(
                &format!(
                    "Expected value to be of type '{}' but found '{}'",
                    expected.get_name(project),
                    actual.get_name(project)
                ),
                span,
            );
        }
        actual
    }
}
