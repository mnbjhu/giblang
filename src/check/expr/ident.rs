use crate::{
    check::{state::CheckState, ty::Ty, NamedExpr},
    fs::{export::Export, project::Project},
    parser::expr::qualified_name::SpannedQualifiedName,
};

pub fn check_ident<'module>(
    state: &mut CheckState<'module>,
    ident: &SpannedQualifiedName,
    project: &'module Project,
) -> Ty<'module> {
    let expr = state.get_path(ident, project, true);
    if let NamedExpr::Imported(Export::Func(f), path) = expr {
        let file = project.get_file(&path[..path.len() - 1]);
        let mut imp_state = CheckState::from_file(file);
        imp_state.import_all(&file.ast, project);
        // TODO: Check generics
        let _ = f.generics.check(project, &mut imp_state, false);
        let receiver = f
            .receiver
            .as_ref()
            .map(|(rec, _)| rec.check(project, &mut imp_state, false))
            .map(Box::new);

        let args = f
            .args
            .iter()
            .map(|(arg, _)| arg.ty.0.check(project, &mut imp_state, false))
            .collect();

        let ret = f
            .ret
            .as_ref()
            .map(|(ret, _)| ret.check(project, &mut imp_state, false))
            .map(Box::new)
            .unwrap_or(Box::new(Ty::Tuple(vec![])));

        return Ty::Function {
            receiver,
            args,
            ret,
        };
    }
    let ty: Ty = expr.into();
    if let Ty::Named { .. } = ty {
        // TODO: Fix primitive meta types
        Ty::Meta(Box::new(ty))
    } else {
        ty
    }
}
