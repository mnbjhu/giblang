use crate::{
    check::{state::CheckState, NamedExpr},
    fs::project::Project,
    parser::common::type_::NamedType,
    ty::{Generic, Ty},
};

impl NamedType {
    pub fn check<'module>(
        &'module self,
        project: &'module Project,
        state: &mut CheckState<'module>,
        print_errors: bool,
    ) -> Ty<'module> {
        let def = state.get_path(&self.name, project, print_errors);
        match &def {
            NamedExpr::Imported(name, path) => {
                if name.valid_type() {
                    let mut args = vec![];
                    let generics = name.generic_args();
                    if generics.0.len() != self.args.len() {
                        state.error(
                            &format!(
                                "Expected {} type parameters but found {}",
                                generics.0.len(),
                                self.args.len()
                            ),
                            self.name.last().unwrap().1,
                        );
                        return Ty::Unknown;
                    }
                    let iter = generics.0.iter().zip(&self.args);
                    let file = project.get_file(&path[0..path.len() - 1]);
                    let mut im_state = CheckState::from_file(file);
                    for (def, (arg, span)) in iter {
                        let ty = arg.check(project, state, print_errors);
                        if let Some(super_) = &def.0.super_ {
                            let super_ = super_.0.check(project, &mut im_state, false);
                            if !ty.is_instance_of(&super_, project) && print_errors {
                                state.error(
                                    &format!("Expected type '{super_}' but found '{ty}'"),
                                    *span,
                                )
                            }
                        }
                        args.push(ty);
                    }
                    Ty::Named {
                        name: name.clone(),
                        args,
                    }
                } else {
                    state.error(
                        "Type must be a 'struct', 'enum' or 'trait'",
                        self.name.last().unwrap().1,
                    );
                    Ty::Unknown
                }
            }
            NamedExpr::Variable(_) => {
                state.error("Variable cannot be a type", self.name.last().unwrap().1);
                Ty::Unknown
            }
            NamedExpr::GenericArg {
                super_,
                variance,
                name,
            } => Ty::Generic(Generic {
                variance: *variance,
                super_: Box::new(super_.clone()),
                name: name.clone(),
            }),
            NamedExpr::Prim(p) => Ty::Prim(p.clone()),
            NamedExpr::Unknown => Ty::Unknown,
        }
    }
}
