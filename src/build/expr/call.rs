use crate::{
    check::state::{CheckState, FoundItem},
    parser::expr::{call::Call, Expr},
    project::decl::{struct_::StructDecl, Decl},
};

use super::ExprKind;

impl Call {
    pub fn build(&self, state: &mut CheckState, kind: &ExprKind) -> String {
        if let Expr::Ident(ident) = &self.name.0.as_ref() {
            if let FoundItem::Decl(id) = state.get_name(ident) {
                if id == 6 {
                    return kind.basic_apply(format!(
                        "fmt.Print({})",
                        self.args
                            .iter()
                            .map(|arg| arg.0.build(state, &ExprKind::Inline))
                            .collect::<Vec<String>>()
                            .join(", ")
                    ));
                }
                let decl = state.project.get_decl(id);
                if let Decl::Struct { body, .. } = decl {
                    let mut text = format!("T{id}{{");
                    if let StructDecl::Fields(fields) = body {
                        for (arg, expr) in fields.iter().zip(&self.args) {
                            text.push_str(&format!(
                                "{}: {},",
                                arg.0,
                                expr.0.build(state, &ExprKind::Inline)
                            ));
                        }
                    } else {
                        todo!()
                    }
                    text.push('}');
                    return kind.basic_apply(text);
                };
            };
        };
        kind.basic_apply(format!(
            "{}({})",
            self.name.0.build(state, kind),
            self.args
                .iter()
                .map(|arg| arg.0.build(state, &ExprKind::Inline))
                .collect::<Vec<String>>()
                .join(", ")
        ))
    }
}
