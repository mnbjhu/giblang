use std::collections::{HashMap, HashSet};

use crate::{
    check::state::CheckState, parser::top::impl_::Impl, project::decl::Decl, ty::Ty, util::Span,
};

impl Impl {
    pub fn build(&self, state: &mut CheckState) -> String {
        state.enter_scope();
        let mut text = String::new();
        let ty = self.for_.0.check(state);
        let trait_ = self.trait_.0.check(state);
        let mut trait_funcs = HashMap::new();
        if let Ty::Named { name, .. } = &trait_ {
            let decl = state.project.get_decl(*name);
            if let Decl::Trait { body, .. } = decl {
                for func in body {
                    let decl = state.project.get_decl(*func);
                    trait_funcs.insert(decl.name(), *func);
                }
            }
        } else {
            unreachable!()
        }

        state.add_self_ty(ty, Span::splat(0));
        for func in &self.body {
            let trait_func = trait_funcs.get(&func.0.name.0).copied();
            text.push_str(&func.0.build(state, trait_func));
        }
        state.exit_scope();
        text
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::{build::build, run::run};

    #[test]
    fn run_test() {
        build();
    }
}
