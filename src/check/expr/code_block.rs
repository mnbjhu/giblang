use crate::{
    check::state::CheckState, parser::expr::code_block::CodeBlock, project::Project, ty::Ty,
};

pub fn check_code_block(state: &mut CheckState, block: &CodeBlock, project: &Project) -> Ty {
    state.enter_scope();
    let mut ret = Ty::Unknown;
    for (stmt, _) in block {
        ret = stmt.check(project, state);
    }
    state.exit_scope();
    ret
}

pub fn check_code_block_is(
    state: &mut CheckState,
    expected: &Ty,
    block: &CodeBlock,
    project: &Project,
) -> Ty {
    if block.is_empty() {
        return Ty::Tuple(vec![]);
    }
    state.enter_scope();
    for (stmt, _) in &block[0..block.len() - 1] {
        stmt.check(project, state);
    }
    let last = block.last().unwrap();

    let actual = last.0.expect_is_instance(expected, project, state, last.1);
    state.exit_scope();
    actual
}
