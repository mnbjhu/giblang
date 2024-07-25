use crate::{
    check::state::CheckState, parser::expr::qualified_name::SpannedQualifiedName, project::Project,
    ty::Ty,
};

pub fn check_ident(state: &mut CheckState, path: &SpannedQualifiedName, _: &Project) -> Ty {
    if path.len() == 1 {
        if let Some(ty) = state.get_variable(&path[0].0) {
            return ty.clone();
        } else if let Some(generic) = state.get_generic(&path[0].0) {
            return Ty::Meta(Box::new(Ty::Generic(generic.clone())));
        }
    }
    // TODO: Check for functions and constructors
    todo!("Create default ty for decl");
    // Ty::Meta(...)
}

pub fn check_ident_is(
    state: &mut CheckState<'_>,
    ident: &SpannedQualifiedName,
    expected: &Ty,
    project: &Project,
) -> Ty {
    let actual = check_ident(state, ident, project);
    if !actual.is_instance_of(expected, project) {
        state.error(
            &format!("Expected value to be of type '{expected}' but found '{actual}'",),
            ident.last().unwrap().1,
        )
    }
    // TODO: Consider whether to pass through or use Ty::Unknown
    actual
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
// fn get_function_ty<'module>(
//     project: &'module Project,
//     path: &[String],
//     f: &'module crate::parser::top::func::Func,
// ) -> Ty<'module> {
//     let file = project.get_file(&path[..path.len() - 1]);
//     let mut imp_state = CheckState::from_file(file);
//     imp_state.import_all(&file.ast, project);
//     // TODO: Check generics
//     let _ = f.generics.check(project, &mut imp_state, false);
//     let receiver = f
//         .receiver
//         .as_ref()
//         .map(|(rec, _)| rec.check(project, &mut imp_state, false))
//         .map(Box::new);
//
//     let args = f
//         .args
//         .iter()
//         .map(|(arg, _)| arg.ty.0.check(project, &mut imp_state, false))
//         .collect();
//
//     let ret = f
//         .ret
//         .as_ref()
//         .map(|(ret, _)| ret.check(project, &mut imp_state, false))
//         .map(Box::new)
//         .unwrap_or(Box::new(Ty::Tuple(vec![])));
//
//     Ty::Function {
//         receiver,
//         args,
//         ret,
//     }
// }
