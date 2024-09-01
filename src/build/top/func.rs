use crate::{build::expr::ExprKind, check::state::CheckState, parser::top::func::Func};

impl Func {
    pub fn build(&self, state: &mut CheckState) -> String {
        assert!(self.receiver.is_none(), "Not implemented yet!");
        state.enter_scope();
        let name = if self.name.0 == "main" {
            "main".to_string()
        } else {
            format!("T{}", self.id)
        };
        let mut res = format!("func {name}(");
        for (i, arg) in self.args.iter().enumerate() {
            if i != 0 {
                res.push_str(", ");
            }
            res.push_str(&arg.0.build(state));
        }
        res.push_str(") ");
        if let Some(ret) = &self.ret {
            res.push_str(&ret.0.build(state));
        }
        res.push_str(" {");
        if let Some(body) = &self.body {
            let last = body.len() - 1;
            for (index, stmt) in body.iter().enumerate() {
                if index == last {
                    if self.ret.is_some() {
                        println!("Building ret {:?}", stmt.0);
                        res.push_str(&stmt.0.build(state, &ExprKind::Return));
                    } else {
                        res.push_str(&stmt.0.build(state, &ExprKind::Inline));
                    }
                } else {
                    res.push_str(&stmt.0.build(state, &ExprKind::Inline));
                }
                res.push('\n');
            }
        }
        res.push('}');
        state.exit_scope();
        res
    }
}
