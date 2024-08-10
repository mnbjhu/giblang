use crate::{
    check::state::CheckState,
    parser::expr::Expr,
    project::Project,
    ty::Ty,
    util::{Span, Spanned},
};

type Tuple = Vec<Spanned<Expr>>;

pub fn check_tuple<'proj>(
    values: &Tuple,
    project: &'proj Project,
    state: &mut CheckState<'proj>,
) -> Ty {
    Ty::Tuple(
        values
            .iter()
            .map(|value| value.0.check(project, state))
            .collect(),
    )
}
pub fn check_tuple_is<'proj>(
    state: &mut CheckState<'proj>,
    expected: &Ty,
    tuple: &Tuple,
    project: &'proj Project,
    span: Span,
) -> Ty {
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
            state.simple_error(
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
        state.simple_error(
            &format!(
                "Expected value to be of type '{}' but found '{}'",
                expected.get_name(state),
                actual.get_name(state),
            ),
            span,
        );
        actual
    }
}
