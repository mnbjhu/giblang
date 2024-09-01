use crate::{check::state::CheckState, parser::expr::property::Property};

use super::ExprKind;

impl Property {
    pub fn build(&self, state: &mut CheckState, kind: &ExprKind) -> String {
        let expr = self.expr.0.build(state, kind);
        format!("{}.{}", expr, self.name.0)
    }
}
