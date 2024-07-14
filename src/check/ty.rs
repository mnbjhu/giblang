use std::fmt::Display;

use crate::{
    fs::{export::Export, project::Project},
    parser::common::{type_::Type, variance::Variance},
};

use super::{CheckState, NamedExpr};

#[derive(Clone)]
pub enum Ty<'module> {
    Any,
    Unknown,
    Named {
        name: Export<'module>,
        args: Vec<Ty<'module>>,
    },
    Generic {
        name: String,
        variance: Variance,
        super_: Box<Ty<'module>>,
    },
    Prim(PrimTy),
}

impl Display for Ty<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ty::Any => write!(f, "Any"),
            Ty::Unknown => write!(f, "Unknown"),
            Ty::Named { name, args } => {
                let name = name.name();
                write!(f, "{}", name)?;
                if !args.is_empty() {
                    write!(f, "[")?;
                    let txt = args
                        .iter()
                        .map(|ty| ty.to_string())
                        .collect::<Vec<_>>()
                        .join(", ");
                    write!(f, "{}", txt)?;
                    write!(f, "]")?;
                }
                Ok(())
            }
            Ty::Generic {
                name,
                variance,
                super_,
            } => {
                write!(f, "{variance}{name}: {super_}")
            }
            Ty::Prim(p) => match p {
                PrimTy::String => write!(f, "String"),
                PrimTy::Bool => write!(f, "Bool"),
                PrimTy::Float => write!(f, "Float"),
                PrimTy::Int => write!(f, "Int"),
                PrimTy::Char => write!(f, "Char"),
            },
        }
    }
}

impl<'module> Ty<'module> {
    pub fn is_instance_of(&'module self, other: &Ty<'module>, project: &Project) -> bool {
        if self.equals(other) {
            return true;
        }
        match (&self, other) {
            (Ty::Unknown, _) => true,
            (_, Ty::Unknown) => true,
            (Ty::Any, _) => false,
            (_, Ty::Any) => true,
            (
                Ty::Named { name, args },
                Ty::Named {
                    name: other_name,
                    args: other_args,
                },
            ) => {
                if name.id() == other_name.id() {
                    args.len() == other_args.len()
                        && args
                            .iter()
                            .zip(other_args)
                            .zip(name.generic_args().0.iter())
                            .all(|((first, second), def)| match def.0.variance {
                                Variance::Invariant => first.equals(second),
                                Variance::Covariant => first.is_instance_of(second, project),
                                Variance::Contravariant => second.is_instance_of(first, project),
                            })
                } else {
                    if let Some(impls) = &name.impls() {
                        if let Some(ty) = impls.iter().find_map(|impl_| impl_.map(self, project)) {
                            if ty.is_instance_of(other, project) {
                                return true;
                            }
                        }
                    }
                    false
                }
            }
            (Ty::Generic { super_, .. }, _) => super_.is_instance_of(other, project),
            // (_, Ty::Generic { super_, .. }) => super_.is_instance_of(other, project),
            _ => false,
        }
    }

    pub fn equals(&'module self, other: &Ty<'module>) -> bool {
        match (&self, other) {
            (Ty::Any, Ty::Any) => true,
            (Ty::Unknown, Ty::Unknown) => true,
            (Ty::Prim(s), Ty::Prim(o)) => s == o,
            (
                Ty::Generic {
                    variance, super_, ..
                },
                Ty::Generic {
                    variance: other_variance,
                    super_: other_super,
                    ..
                },
            ) => super_.equals(other_super) && variance == other_variance,
            (
                Ty::Named { name, args },
                Ty::Named {
                    name: other_name,
                    args: other_args,
                },
            ) => {
                name.id() == other_name.id()
                    && args.len() == other_args.len()
                    && args
                        .iter()
                        .zip(other_args)
                        .all(|(first, second)| first.equals(second))
            }
            _ => false,
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum PrimTy {
    String,
    Bool,
    Float,
    Int,
    Char,
}

impl Type {
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
            } => Ty::Generic {
                variance: *variance,
                super_: Box::new(super_.clone()),
                name: name.clone(),
            },
            NamedExpr::Prim(p) => Ty::Prim(p.clone()),
            NamedExpr::Unknown => Ty::Unknown,
        }
    }
}
