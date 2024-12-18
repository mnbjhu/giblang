use crate::{
    db::decl::{func::Function, Decl, DeclKind},
    parser::top::func::Func,
    resolve::state::ResolveState,
    ty::Ty,
};

impl Func {
    pub fn resolve<'db>(&self, state: &mut ResolveState<'db>, virtual_: bool) -> Decl<'db> {
        let name = self.name.clone();
        let generics = self.generics.0.resolve(state);
        let receiver = self.receiver.as_ref().map(|(rec, _)| rec.resolve(state));
        let args = self.args.iter().map(|arg| arg.0.resolve(state)).collect();
        let ret = self
            .ret
            .as_ref()
            .map_or(Ty::unit(), |(ret, _)| ret.resolve(state));
        let kind = DeclKind::Function(Function {
            name: self.name.0.clone(),
            generics,
            receiver,
            args,
            ret,
            required: self.body.is_none(),
            body: self.body.clone().unwrap_or_default(),
            virtual_,
        });
        Decl::new(
            state.db,
            name.0,
            name.1,
            kind,
            Some(state.file_data),
            state.module_path(),
        )
    }
}
