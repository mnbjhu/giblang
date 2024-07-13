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
        variance: Variance,
        super_: Box<Ty<'module>>,
    },
    Prim(PrimTy),
}

#[derive(Clone)]
pub enum PrimTy {
    String,
    Bool,
    Float,
    Int,
}

impl Type {
    pub fn check<'module>(
        &'module self,
        project: &'module Project,
        state: &mut CheckState<'module>,
        print_errors: bool,
    ) -> Ty<'module> {
        let def = state.get_path(&self.name, project, print_errors);
        match def {
            NamedExpr::Export(name) => {
                if name.valid_type() {
                    let mut args = vec![];
                    for (arg, _) in &self.args {
                        let ty = arg.check(project, state, print_errors);
                        args.push(ty);
                    }
                    Ty::Named { name, args }
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
            NamedExpr::GenericArg { super_, variance } => Ty::Generic {
                variance,
                super_: Box::new(super_),
            },
            NamedExpr::Prim(p) => Ty::Prim(p),
            NamedExpr::Unknown => Ty::Unknown,
        }
    }
}
