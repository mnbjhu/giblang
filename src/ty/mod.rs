use salsa::Update;

use crate::{
    check::state::CheckState,
    db::{input::Db, modules::ModulePath},
    parser::common::variance::Variance,
    util::Spanned,
};

pub mod imply;
pub mod is_instance;
pub mod name;
pub mod parameterize;
pub mod prim;

#[derive(Clone, Debug, Eq, PartialEq, Update, Hash)]
pub struct Generic<'db> {
    pub name: Spanned<String>,
    pub variance: Variance,
    pub super_: Box<Ty<'db>>,
}

impl<'db> Generic<'db> {
    pub fn new(name: Spanned<String>) -> Generic<'db> {
        Generic {
            name,
            variance: Variance::Invariant,
            super_: Box::new(Ty::Any),
        }
    }

    pub fn get_name(&self, db: &'db dyn Db, state: &CheckState) -> String {
        if let Ty::Any = self.super_.as_ref() {
            format!("{}{}", self.variance, self.name.0)
        } else {
            format!(
                "{}{}: {}",
                self.variance,
                self.name.0,
                self.super_.get_name(db, state)
            )
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default, Update, Eq, Hash)]
pub struct FuncTy<'db> {
    pub receiver: Option<Box<Ty<'db>>>,
    pub args: Vec<Ty<'db>>,
    pub ret: Box<Ty<'db>>,
}

#[derive(Clone, Debug, PartialEq, Default, Update, Hash, Eq)]
pub enum Ty<'db> {
    Any,
    #[default]
    Unknown,
    Named {
        name: ModulePath<'db>,
        args: Vec<Ty<'db>>,
    },
    TypeVar {
        id: u32,
    },
    Generic(Generic<'db>),
    Meta(Box<Ty<'db>>),
    Function(FuncTy<'db>),
    Tuple(Vec<Ty<'db>>),
    Sum(Vec<Ty<'db>>),
}

// #[cfg(test)]
// mod tests {
//     use crate::parser::common::variance::Variance;
//     use crate::project::{check_test_state, Project};
//     use crate::ty::{Generic, Ty};
//     use crate::util::Span;
//
// #[test]
// fn simple_name() {
//     let project = Project::check_test();
//     let state = check_test_state(&project);
//     let gen = Generic::new(("T".to_string(), Span::splat(0)));
//     let name = gen.get_name(&state);
//     assert_eq!(name, "T");
// }
//
// #[test]
// fn name_with_super() {
//     let project = Project::check_test();
//     let state = check_test_state(&project);
//     let gen = Generic {
//         name: ("T".to_string(), Span::splat(0)),
//         variance: Variance::Invariant,
//         super_: Box::new(Ty::int()),
//     };
//     let name = gen.get_name(&state);
//     assert_eq!(name, "T: Int");
// }
//
// #[test]
// fn name_with_variance() {
//     let project = Project::check_test();
//     let state = check_test_state(&project);
//     let gen = Generic {
//         name: ("T".to_string(), Span::splat(0)),
//         variance: Variance::Covariant,
//         super_: Box::new(Ty::Any),
//     };
//     let name = gen.get_name(&state);
//     assert_eq!(name, "out T");
// }
// }
