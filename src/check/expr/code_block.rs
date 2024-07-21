use crate::{
    check::state::CheckState, parser::expr::code_block::CodeBlock, project::Project, ty::Ty,
};

pub fn check_code_block<'proj>(
    state: &mut CheckState<'proj>,
    block: &'proj CodeBlock,
    project: &'proj Project,
) -> Ty {
    state.enter_scope();
    let mut ret = Ty::Unknown;
    for (stmt, _) in block {
        ret = stmt.check(project, state);
    }
    state.exit_scope();
    ret
}

pub fn check_code_block_is<'proj>(
    state: &mut CheckState<'proj>,
    expected: &Ty,
    block: &'proj CodeBlock,
    project: &'proj Project,
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
