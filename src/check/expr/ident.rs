use crate::{
    check::{state::CheckState, NamedExpr},
    fs::{export::Export, project::Project},
    parser::{
        common::generic_args::GenericArgs, expr::qualified_name::SpannedQualifiedName,
        top::struct_body::StructBody,
    },
    ty::Ty,
};

pub fn check_ident<'module>(
    state: &mut CheckState<'module>,
    ident: &SpannedQualifiedName,
    project: &'module Project,
) -> Ty<'module> {
    let expr = state.get_path(ident, project, true);
    if let NamedExpr::Variable(ty) = expr {
        return ty.clone();
    }
    if let NamedExpr::Imported(ex, path) = &expr {
        match ex {
            Export::Func(f) => {
                let ret = get_function_ty(project, path, f);
                return ret;
            }
            Export::Struct(s) => {
                let ret = get_body_ty(project, path, &s.generics.0, &s.body, expr.clone().into());
                return ret;
            }
            Export::Member { parent, member } => {
                let ret = get_body_ty(
                    project,
                    &path[..path.len() - 1],
                    &parent.generics.0,
                    &member.body,
                    Ty::Named {
                        name: Export::Enum(parent),
                        args: vec![],
                    },
                );
                return ret;
            }
            _ => (),
        }
    }
    let ty: Ty = expr.into();
    if let Ty::Unknown = ty {
        Ty::Unknown
    } else {
        Ty::Meta(Box::new(ty))
    }
}

fn get_body_ty<'module>(
    project: &'module Project,
    path: &[String],
    generics: &'module GenericArgs,
    body: &'module StructBody,
    return_ty: Ty<'module>,
) -> Ty<'module> {
    let file = project.get_file(&path[..path.len() - 1]);
    let mut imp_state = CheckState::from_file(file);
    imp_state.import_all(&file.ast, project);
    // TODO: Check generics
    let _ = generics.check(project, &mut imp_state, false);
    let ret = match &body {
        StructBody::None => return_ty,
        StructBody::Tuple(fields) => {
            let args = fields
                .iter()
                .map(|ty| ty.0.check(project, &mut imp_state, false))
                .collect();
            Ty::Function {
                receiver: None,
                args,
                ret: Box::new(return_ty),
            }
        }
        StructBody::Fields(fields) => {
            let args = fields
                .iter()
                .map(|ty| ty.0.ty.0.check(project, &mut imp_state, false))
                .collect();
            Ty::Function {
                receiver: None,
                args,
                ret: Box::new(return_ty),
            }
        }
    };
    ret
}

fn get_function_ty<'module>(
    project: &'module Project,
    path: &[String],
    f: &'module crate::parser::top::func::Func,
) -> Ty<'module> {
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

    Ty::Function {
        receiver,
        args,
        ret,
    }
}
