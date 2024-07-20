use crate::{
    check::state::CheckState, fs::project::Project, parser::expr::code_block::CodeBlock, ty::Ty,
};

pub fn check_code_block<'module>(
    state: &mut CheckState<'module>,
    block: &'module CodeBlock,
    project: &'module Project,
) -> Ty<'module> {
    state.enter_scope();
    let mut ret = Ty::Unknown;
    for (stmt, _) in block {
        ret = stmt.check(project, state);
    }
    state.exit_scope();
    ret
}

pub fn check_code_block_is<'module>(
    state: &mut CheckState<'module>,
    expected: &Ty<'module>,
    block: &'module CodeBlock,
    project: &'module Project,
) -> Ty<'module> {
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
