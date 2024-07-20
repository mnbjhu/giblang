use crate::{
    check::state::CheckState,
    fs::project::Project,
    parser::expr::Expr,
    ty::Ty,
    util::{Span, Spanned},
};

type Tuple = Vec<Spanned<Expr>>;

pub fn check_tuple<'module>(
    values: &'module Tuple,
    project: &'module Project,
    state: &mut CheckState<'module>,
) -> Ty<'module> {
    Ty::Tuple(
        values
            .iter()
            .map(|value| value.0.check(project, state))
            .collect(),
    )
}
pub fn check_tuple_is<'module>(
    state: &mut CheckState<'module>,
    expected: &Ty<'module>,
    tuple: &'module Tuple,
    project: &'module Project,
    span: Span,
) -> Ty<'module> {
    if let Ty::Tuple(ex) = expected {
        if ex.len() == tuple.len() {
            let v = ex
                .iter()
                .zip(tuple)
                .map(|(ex, ac)| ac.0.expect_instance_of(ex, project, state, span))
                .collect();
            Ty::Tuple(v)
        } else {
            let actual = check_tuple(tuple, project, state);
            state.error(
                &format!(
                    "Expected a tuple of length {} but found one of length {}",
                    ex.len(),
                    tuple.len()
                ),
                span,
            );
            actual
        }
    } else {
        let actual = check_tuple(tuple, project, state);
        state.error(
            &format!("Expected value to be of type '{expected}' but found '{actual}'",),
            span,
        );
        actual
    }
}
