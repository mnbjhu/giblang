use crate::{check::state::CheckState, parser::common::variance::Variance, util::Spanned};

pub mod imply;
pub mod is_instance;
pub mod name;
pub mod parameterize;
pub mod prim;

#[derive(Clone, Debug, PartialEq)]
pub struct Generic {
    pub name: Spanned<String>,
    pub variance: Variance,
    pub super_: Box<Ty>,
}

impl Generic {
    pub fn new(name: Spanned<String>) -> Generic {
        Generic {
            name,
            variance: Variance::Invariant,
            super_: Box::new(Ty::Any),
        }
    }

    pub fn get_name(&self, state: &CheckState) -> String {
        if let Ty::Any = self.super_.as_ref() {
            format!("{}{}", self.variance, self.name.0)
        } else {
            format!(
                "{}{}: {}",
                self.variance,
                self.name.0,
                self.super_.get_name(state)
            )
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Ty {
    Any,
    #[default]
    Unknown,
    Named {
        name: u32,
        args: Vec<Ty>,
    },
    TypeVar {
        id: u32,
    },
    Generic(Generic),
    Meta(Box<Ty>),
    Function {
        receiver: Option<Box<Ty>>,
        args: Vec<Ty>,
        ret: Box<Ty>,
    },
    Tuple(Vec<Ty>),
    Sum(Vec<Ty>),
}

#[cfg(test)]
mod tests {
    use crate::parser::common::variance::Variance;
    use crate::project::{check_test_state, Project};
    use crate::ty::{Generic, Ty};
    use crate::util::Span;

    #[test]
    fn simple_name() {
        let project = Project::check_test();
        let state = check_test_state(&project);
        let gen = Generic::new(("T".to_string(), Span::splat(0)));
        let name = gen.get_name(&state);
        assert_eq!(name, "T");
    }

    #[test]
    fn name_with_super() {
        let project = Project::check_test();
        let state = check_test_state(&project);
        let gen = Generic {
            name: ("T".to_string(), Span::splat(0)),
            variance: Variance::Invariant,
            super_: Box::new(Ty::int()),
        };
        let name = gen.get_name(&state);
        assert_eq!(name, "T: Int");
    }

    #[test]
    fn name_with_variance() {
        let project = Project::check_test();
        let state = check_test_state(&project);
        let gen = Generic {
            name: ("T".to_string(), Span::splat(0)),
            variance: Variance::Covariant,
            super_: Box::new(Ty::Any),
        };
        let name = gen.get_name(&state);
        assert_eq!(name, "out T");
    }
}
