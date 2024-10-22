use crate::{
    check::{
        err::{is_not_instance::IsNotInstance, CheckError},
        state::CheckState,
    },
    parser::expr::Expr,
    ty::Ty,
    util::{Span, Spanned},
};

type Tuple = Vec<Spanned<Expr>>;

pub fn check_tuple<'db>(values: &Tuple, state: &mut CheckState<'db>) -> Ty<'db> {
    Ty::Tuple(values.iter().map(|value| value.0.check(state)).collect())
}
pub fn check_tuple_is<'db>(
    state: &mut CheckState<'db>,
    expected: &Ty<'db>,
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
        let found = check_tuple(tuple, state);
        state.error(CheckError::IsNotInstance(IsNotInstance {
            expected: expected.get_name(state),
            found: found.get_name(state),
            span,
            file: state.file_data,
        }));
    }
}
