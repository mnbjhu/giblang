use crate::{
    check::state::CheckState, parser::top::func::Func, project::decl::Decl,
    resolve::state::ResolveState, ty::Ty,
};

impl Func {
    pub fn resolve(&self, state: &mut ResolveState) -> Decl {
        let name = self.name.clone();
        let generics = self.generics.resolve(state);
        let receiver = self.receiver.as_ref().map(|(rec, _)| rec.resolve(state));
        let args = self.args.iter().map(|arg| arg.0.resolve(state)).collect();
        let ret = self
            .ret
            .as_ref()
            .map_or(Ty::unit(), |(ret, _)| ret.resolve(state));
        Decl::Function {
            name,
            generics,
            receiver,
            args,
            ret,
        }
    }
}
