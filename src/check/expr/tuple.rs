use crate::{
    check::state::CheckState,
    parser::expr::Expr,
    ty::Ty,
    util::{Span, Spanned},
};

type Tuple = Vec<Spanned<Expr>>;

pub fn check_tuple<'proj>(values: &Tuple, state: &mut CheckState<'proj>) -> Ty {
    Ty::Tuple(values.iter().map(|value| value.0.check(state)).collect())
}
pub fn check_tuple_is<'proj>(
    state: &mut CheckState<'proj>,
    expected: &Ty,
    tuple: &Tuple,
    span: Span,
) {
    if let Ty::Tuple(ex) = expected {
        if ex.len() == tuple.len() {
            ex.iter()
                .zip(tuple)
                .for_each(|(ex, ac)| ac.0.expect_instance_of(ex, state, span));
        } else {
            for value in tuple {
                value.0.check(state);
            }
            state.simple_error(
                &format!(
                    "Expected a tuple of length {} but found one of length {}",
                    ex.len(),
                    tuple.len()
                ),
                span,
            );
        }
    } else {
        check_tuple(tuple, state);
        todo!("TODO: Add expected a tuple error");
    }
}
