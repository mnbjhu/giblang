use crate::{
    build::expr::ExprKind,
    check::state::CheckState,
    parser::{
        common::pattern::Pattern,
        stmt::let_::LetStatement,
    },
};

impl LetStatement {
    pub fn build(&self, state: &mut CheckState) -> String {
        // TODO: Think about how to do other pattern types
        self.check(state);
        let ty = if let Some(explicit) = &self.ty {
            explicit.0.check(state)
        } else {
            self.value.0.check(state)
        };
        if let Pattern::Name(name) = &self.pattern.0 {
            format!(
                "var {} {}\n{}",
                name,
                ty.build(state),
                self.value.0.build(state, &ExprKind::Assign(name.clone()))
            )
        } else {
            todo!()
        }
    }
}

// pub fn build_assign(pat: &Pattern, expr: &Expr, state: &mut CheckState) -> String {
//     match pat {
//         Pattern::Name(name) => expr.build(state, &ExprKind::Assign(name.clone())),
//         Pattern::Struct { name, fields } => {}
//         Pattern::UnitStruct(_) => todo!(),
//         Pattern::TupleStruct { name, fields } => todo!(),
//     }
// }
// fn build_assign_str(pat: &Pattern, expr: String, state: &mut CheckState) -> String {
//     match pat {
//         Pattern::Name(name) => format!(),
//         Pattern::Struct { fields, .. } => {
//             let mut ret = String::new();
//             for field in fields {
//                 match field.0 {
//                     StructFieldPattern::Implied(name) => {}
//                     StructFieldPattern::Explicit { field, pattern } => todo!(),
//                 }
//             }
//         }
//         Pattern::UnitStruct(_) => todo!(),
//         Pattern::TupleStruct { name, fields } => todo!(),
//     }
// }
