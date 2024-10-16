use std::collections::HashMap;

use crate::{
    check::state::CheckState, db::modules::ModuleData,
    parser::expr::qualified_name::SpannedQualifiedName, ty::Ty,
};

pub fn check_ident<'db>(state: &mut CheckState<'_, 'db>, path: &SpannedQualifiedName) -> Ty<'db> {
    if path.len() == 1 {
        if let Some(var) = state.get_variable(&path[0].0) {
            return var.ty;
        } else if let Some(generic) = state.get_generic(&path[0].0).cloned() {
            return Ty::Meta(Box::new(Ty::Generic(generic)));
        }
    }

    if let Some(module) = state.get_module_with_error(path) {
        match module.content(state.db) {
            ModuleData::Package(_) => Ty::unit(),
            ModuleData::Export(decl) => decl
                .get_ty(module.path(state.db), state)
                .inst(&mut HashMap::new(), state),
        }
    } else {
        Ty::Unknown
    }
}

pub fn check_ident_is<'db>(
    state: &mut CheckState<'_, 'db>,
    ident: &SpannedQualifiedName,
    expected: &Ty<'db>,
) {
    let actual = check_ident(state, ident);
    let span = ident.last().unwrap().1;
    actual.expect_is_instance_of(expected, state, false, span);
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
