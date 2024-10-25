use crate::{
    check::state::CheckState, parser::expr::lambda::{Lambda, LambdaParam}, ty::{FuncTy, Ty}, util::Span
};

use super::code_block::check_code_block;

impl<'db> Lambda {
    pub fn check(&self, state: &mut CheckState<'db>) -> Ty<'db> {
        let args = self.args.iter().map(|arg| arg.0.check(state)).collect();
        let ret = check_code_block(state, &self.body.0);
        Ty::Function(FuncTy { receiver: None, args, ret: Box::new(ret) })
    }

    pub fn expected_instance_of(
        &self,
        expected: &Ty<'db>,
        state: &mut CheckState<'db>,
        span: Span,
    ) {
        let ty = self.check(state);
        ty.expect_is_instance_of(expected, state, false, span);
    }
}

impl<'db> LambdaParam {
    pub fn check(&self, state: &mut CheckState<'db>) -> Ty<'db> {
        if let Some(ty) = &self.ty {
            let expected = ty.0.check(state);
            self.pattern.0.check(state, &expected);
            expected
        } else {
            let id = state.type_state.new_type_var(self.pattern.1, state.file_data);
            let type_var = Ty::TypeVar { id };
            self.pattern.0.check(state, &type_var);
            type_var

        }
    }

    pub fn expect_instance_of(&self, expected: &Ty<'db>, state: &mut CheckState<'db>, span: Span) {
        let ty = self.check(state);
        ty.expect_is_instance_of(expected, state, false, self.ty.as_ref().map_or(self.pattern.1, |ty|ty.1));
    }
}
