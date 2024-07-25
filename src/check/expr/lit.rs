use crate::{
    check::state::CheckState,
    lexer::literal::Literal,
    project::Project,
    ty::{PrimTy, Ty},
    util::Span,
};

impl From<&Literal> for Ty {
    fn from(value: &Literal) -> Self {
        match value {
            Literal::Int(_) => Ty::Prim(PrimTy::Int),
            Literal::Float(_) => Ty::Prim(PrimTy::Float),
            Literal::String(_) => Ty::Prim(PrimTy::String),
            Literal::Bool(_) => Ty::Prim(PrimTy::Bool),
            Literal::Char(_) => Ty::Prim(PrimTy::Char),
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
            state.error(
                &format!("Expected value to be of type '{expected}' but found '{actual}'",),
                span,
            )
        }
        actual
    }
}
