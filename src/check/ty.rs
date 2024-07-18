use std::fmt::Display;

use crate::{
    fs::{export::Export, project::Project},
    parser::common::{
        type_::{NamedType, Type},
        variance::Variance,
    },
};

use super::{CheckState, NamedExpr};

#[derive(Clone, Debug)]
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
    Meta(Box<Ty<'module>>),
    Function {
        receiver: Option<Box<Ty<'module>>>,
        args: Vec<Ty<'module>>,
        ret: Box<Ty<'module>>,
    },
    Tuple(Vec<Ty<'module>>),
    Sum(Vec<Ty<'module>>),
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
            Ty::Meta(ty) => write!(f, "Type[{}]", ty),
            Ty::Function {
                receiver,
                args,
                ret,
            } => {
                let args = args
                    .iter()
                    .map(|ty| ty.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                if let Some(receiver) = receiver {
                    write!(f, "{}.({}) -> {}", receiver, args, ret)
                } else {
                    write!(f, "({}) -> {}", args, ret)
                }
            }
            Ty::Tuple(tys) => {
                let txt = tys
                    .iter()
                    .map(|ty| ty.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "({})", txt)
            }
            Ty::Sum(tys) => {
                let txt = tys
                    .iter()
                    .map(|ty| ty.to_string())
                    .collect::<Vec<_>>()
                    .join(" + ");
                write!(f, "{}", txt)
            }
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
                } else if let Some(impls) = &name.impls() {
                    impls
                        .iter()
                        .filter_map(|impl_| impl_.map(self, project))
                        .any(|implied| implied.is_instance_of(other, project))
                } else {
                    false
                }
            }
            (_, Ty::Sum(tys)) => tys.iter().all(|other| self.is_instance_of(other, project)),
            (Ty::Tuple(v), Ty::Tuple(other)) => {
                v.len() == other.len()
                    && v.iter()
                        .zip(other)
                        .all(|(s, o)| s.is_instance_of(o, project))
            }
            (Ty::Generic { super_, .. }, _) => super_.is_instance_of(other, project),
            // (_, Ty::Generic { super_, .. }) => super_.is_instance_of(other, project),
            (
                Ty::Function {
                    receiver,
                    args,
                    ret,
                },
                Ty::Function {
                    receiver: other_receiver,
                    args: other_args,
                    ret: other_ret,
                },
            ) => {
                args.len() == other_args.len()
                    && args
                        .iter()
                        .zip(other_args)
                        .all(|(first, second)| second.is_instance_of(first, project))
                    && ret.is_instance_of(other_ret, project)
                    && receiver.as_ref().map_or(true, |r| {
                        other_receiver
                            .as_ref()
                            .map_or(false, |o| o.is_instance_of(r, project))
                    })
            }
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
            (
                Ty::Function {
                    receiver,
                    args,
                    ret,
                },
                Ty::Function {
                    receiver: other_receiver,
                    args: other_args,
                    ret: other_ret,
                },
            ) => {
                receiver.as_ref().map_or(true, |r| {
                    other_receiver.as_ref().map_or(false, |o| r.equals(o))
                }) && args.len() == other_args.len()
                    && args
                        .iter()
                        .zip(other_args)
                        .all(|(first, second)| first.equals(second))
                    && ret.equals(other_ret)
            }
            (Ty::Tuple(tys), Ty::Tuple(other_tys)) => {
                tys.len() == other_tys.len()
                    && tys
                        .iter()
                        .zip(other_tys)
                        .all(|(first, second)| first.equals(second))
            }
            _ => false,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
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
        match &self {
            Type::Named(named) => named.check(project, state, print_errors),
            Type::Tuple(tup) => {
                let mut tys = vec![];
                for (ty, _) in tup {
                    tys.push(ty.check(project, state, print_errors))
                }
                Ty::Tuple(tys)
            }
            Type::Sum(tup) => {
                let mut tys = vec![];
                for (ty, _) in tup {
                    tys.push(ty.check(project, state, print_errors))
                }
                Ty::Sum(tys)
            }
            Type::Function {
                receiver,
                args,
                ret,
            } => Ty::Function {
                receiver: receiver.as_ref().map(|receiver| Box::new(receiver.as_ref().0.check(
                        project,
                        state,
                        print_errors,
                    ))),
                args: args
                    .iter()
                    .map(|r| r.0.check(project, state, print_errors))
                    .collect(),
                ret: Box::new(ret.0.check(project, state, print_errors)),
            },
        }
    }
}

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
