use crate::{
    check::state::CheckState,
    fs::project::Project,
    lexer::literal::Literal,
    ty::{PrimTy, Ty},
    util::Span,
};

impl<'module> From<&Literal> for Ty<'module> {
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
    pub fn expect_instance_of<'module>(
        &'module self,
        expected: &Ty<'module>,
        project: &'module Project,
        state: &mut CheckState<'module>,
        span: Span,
    ) -> Ty<'module> {
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
