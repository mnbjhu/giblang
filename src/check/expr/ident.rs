use crate::{
    check::state::CheckState, parser::expr::qualified_name::SpannedQualifiedName, project::Project,
    ty::Ty, util::Span,
};

pub fn check_ident(state: &mut CheckState, path: &SpannedQualifiedName, project: &Project) -> Ty {
    if path.len() == 1 {
        if let Some(ty) = state.get_variable(&path[0].0) {
            return ty.clone();
        } else if let Some(generic) = state.get_generic(&path[0].0) {
            return Ty::Meta(Box::new(Ty::Generic(generic.clone())));
        }
    }
    if let Some(decl_id) = state.get_decl_with_error(path) {
        let decl = project.get_decl(decl_id);
        decl.get_ty(decl_id, project)
    } else {
        Ty::Unknown
    }
    // Ty::Meta(...)
}

pub fn check_ident_is(
    state: &mut CheckState<'_>,
    ident: &SpannedQualifiedName,
    expected: &Ty,
    project: &Project,
) -> Ty {
    let actual = check_ident(state, ident, project);
    let span = ident.last().unwrap().1;
    check_ty(actual, expected, project, state, span)
}

pub fn check_ty(
    actual: Ty,
    expected: &Ty,
    project: &Project,
    state: &mut CheckState<'_>,
    span: Span,
) -> Ty {
    let implied = actual.imply_generics(expected);
    let new = if let Some(implied) = implied {
        actual.parameterize(&implied)
    } else {
        actual
    };
    if !new.is_instance_of(expected, project) {
        state.simple_error(
            &format!(
                "Expected value to be of type '{}' but found '{}'",
                expected.get_name(project),
                new.get_name(project),
            ),
            span,
        );
    }
    // TODO: Consider whether to pass through or use Ty::Unknown
    new
}

// fn get_body_ty<'module>(
//     project: &'module Project,
//     path: &[String],
//     generics: &'module GenericArgs,
//     body: &'module StructBody,
//     name: Export<'module>,
// ) -> Ty {
//     let file = project.get_file(&path[..path.len() - 1]);
//     let mut imp_state = CheckState::from_file(file);
//     imp_state.import_all(&file.ast, project);
//     // TODO: Check generics
//     let generics = generics.check(project, &mut imp_state, false);
//     let ret = match &body {
//         StructBody::None => Ty::Named {
//             name,
//             args: generics.iter().map(|_| Ty::Unknown).collect(),
//         },
//         StructBody::Tuple(fields) => {
//             let args = fields
//                 .iter()
//                 .map(|ty| ty.0.check(project, &mut imp_state, false))
//                 .collect();
//             Ty::Function {
//                 receiver: None,
//                 args,
//                 ret: Box::new(Ty::Named {
//                     name,
//                     args: generics,
//                 }),
//             }
//         }
//         StructBody::Fields(fields) => {
//             let args = fields
//                 .iter()
//                 .map(|ty| ty.0.ty.0.check(project, &mut imp_state, false))
//                 .collect();
//             Ty::Function {
//                 receiver: None,
//                 args,
//                 ret: Box::new(Ty::Named {
//                     name,
//                     args: generics,
//                 }),
//             }
//         }
//     };
//     ret
// }
//
