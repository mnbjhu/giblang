use crate::{
    check::{
        err::{is_not_instance::IsNotInstance, CheckError},
        state::CheckState,
    },
    parser::expr::Expr,
    ty::Ty,
    util::{Span, Spanned},
};

use super::{ExprIR, ExprIRData};

pub fn check_tuple<'db>(tuple: &Vec<Spanned<Expr>>, state: &mut CheckState<'db>) -> ExprIR<'db> {
    let mut tys = Vec::new();
    let mut exprs = Vec::new();
    for expr in tuple {
        let ir = expr.0.check(state);
        tys.push(ir.ty.clone());
        exprs.push((ir, expr.1));
    }
    ExprIR {
        data: ExprIRData::Tuple(exprs),
        ty: Ty::Tuple(tys),
    }
}

pub fn expect_tuple<'db>(
    tuple: &Vec<Spanned<Expr>>,
    state: &mut CheckState<'db>,
    expected: &Ty<'db>,
    span: Span,
) -> ExprIR<'db> {
    if let Ty::Tuple(ex) = expected {
        if ex.len() == tuple.len() {
            let mut exprs = Vec::new();
            for (ex, ac) in ex.iter().zip(tuple) {
                exprs.push((ac.0.expect(state, ex, ac.1), ac.1));
            }
            let tys = exprs.iter().map(|e| e.0.ty.clone()).collect();
            ExprIR {
                data: ExprIRData::Tuple(exprs),
                ty: Ty::Tuple(tys),
            }
        } else {
            let mut exprs = vec![];
            for value in tuple {
                exprs.push((value.0.check(state), value.1));
            }
            state.simple_error(
                &format!(
                    "Expected a tuple of length {} but found one of length {}",
                    ex.len(),
                    tuple.len()
                ),
                span,
            );
            ExprIR {
                data: ExprIRData::Tuple(exprs),
                ty: Ty::Unknown,
            }
        }
    } else {
        let found = check_tuple(tuple, state);
        state.error(CheckError::IsNotInstance(IsNotInstance {
            expected: expected.get_name(state),
            found: found.ty.get_name(state),
            span,
            file: state.file_data,
        }));
        found
    }
}
