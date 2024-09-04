use crate::{
    build::expr::ExprKind,
    check::state::CheckState,
    parser::{common::type_::Type, top::func::Func},
    ty::Ty,
};

impl Func {
    pub fn build(&self, state: &mut CheckState, func_name: Option<u32>) -> String {
        state.enter_scope();
        let name = if self.name.0 == "main" {
            "main".to_string()
        } else {
            format!("T{}", self.id)
        };
        let mut res = "func ".to_string();
        if let Some(rec) = &self.receiver {
            res.push_str("(self ");
            if let Type::Named(name) = &rec.0 {
                if name.name[0].0 == "Self" {
                    let generic = state.get_generic("Self").cloned().unwrap();
                    let bound = generic.super_.as_ref().build(state);
                    res.push_str(&bound);
                } else {
                    res.push_str(&rec.0.build(state));
                }
            } else {
                res.push_str(&rec.0.build(state));
            }
            res.push_str(") ");
        }

        if let Some(func_name) = func_name {
            res.push_str(&format!("T{}(", func_name));
        } else {
            res.push_str(&format!("{name}("));
        }
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
