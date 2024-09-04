use crate::{check::state::CheckState, parser::top::trait_::Trait, util::Span};

impl Trait {
    pub fn build(&self, state: &mut CheckState) -> String {
        state.enter_scope();
        let self_ty = state.project.get_decl(self.id).get_ty(self.id, state);
        state.add_self_ty(self_ty, Span::splat(0));
        let name = &self.id;
        let mut text = format!("type T{name} interface {{\n");
        for method in &self.body {
            if method.0.receiver.is_some() {
                // TODO: Should only be self
                text.push_str(&format!(
                    "T{name}({args})",
                    name = method.0.id,
                    args = method
                        .0
                        .args
                        .iter()
                        .map(|arg| arg.0.build(state))
                        .collect::<Vec<_>>()
                        .join(", "),
                ));
                if let Some(ret) = &method.0.ret {
                    text.push_str(&format!(" {ret}", ret = ret.0.build(state)));
                }
                text.push('\n');
            }
        }
        text.push('}');
        state.exit_scope();
        text
    }
}
