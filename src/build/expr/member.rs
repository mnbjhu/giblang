use crate::{
    check::state::{CheckState, FoundItem},
    parser::expr::member::MemberCall,
    project::decl::Decl,
    ty::{is_instance::find_function, Ty},
};

use super::ExprKind;

impl MemberCall {
    pub fn build(&self, state: &mut CheckState, kind: &ExprKind) -> String {
        let rec_ty = self.rec.0.check(state);
        if let Ty::Named { name: ty, .. } = rec_ty {
            if let Some(func) = find_function(&self.name.0, ty, state) {
                let decl = state.project.get_decl(func);
                if let Decl::Function { args, ret, .. } = decl {
                    return format!(
                        "{expr}.T{func}({args})",
                        expr = self.rec.0.build(state, kind),
                        args = self
                            .args
                            .iter()
                            .map(|arg| arg.0.build(state, kind))
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                } else {
                    panic!()
                }
            }
        }
        if let FoundItem::Decl(name) = state.get_name(&vec![self.name.clone()]) {
            format!(
                "{expr}.T{name}({args})",
                expr = self.rec.0.build(state, kind),
                args = self
                    .args
                    .iter()
                    .map(|arg| arg.0.build(state, kind))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        } else {
            todo!()
        }
    }
}
