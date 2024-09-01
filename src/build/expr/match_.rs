use crate::{check::state::CheckState, parser::expr::match_::Match};

use super::ExprKind;

// impl Match {
//     pub fn build(&self, state: &mut CheckState, kind: &ExprKind) -> String {
//         let expr = self.expr.0.build(state, kind);
//         let mut ret = format!(
//             r#"
//         expr := {expr}
//         "#
//         );
//         let last = self.arms.len() - 1;
//         for arm in &self.arms {
//
//         }
//     }
// }
