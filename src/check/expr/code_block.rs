use crate::{check::state::CheckState, parser::expr::code_block::CodeBlock, ty::Ty, util::Span};

pub fn check_code_block<'db>(state: &mut CheckState<'db>, block: &CodeBlock) -> Ty<'db> {
    state.enter_scope();
    let mut ret = Ty::unit();
    for (stmt, _) in block {
        ret = stmt.check(state);
    }
    state.exit_scope();
    ret
}

pub fn check_code_block_is<'db>(
    state: &mut CheckState<'db>,
    expected: &Ty<'db>,
    block: &CodeBlock,
    span: Span,
) {
    if block.is_empty() {
        Ty::unit().expect_is_instance_of(expected, state, false, span);
        return;
    }
    state.enter_scope();
    for (stmt, _) in &block[0..block.len() - 1] {
        stmt.check(state);
    }
    let last = block.last().unwrap();

    last.0.expect_is_instance(expected, state, last.1);
    state.exit_scope();
}
