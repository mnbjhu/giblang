use std::collections::HashMap;

use super::Ty;

// TODO: This should use unique ids instead of the String names for generic type args
impl Ty {
    pub fn parameterize(&self, generics: &HashMap<String, Ty>) -> Ty {
        match self {
            Ty::Generic(arg) => generics.get(&arg.name.0).unwrap_or(self).clone(),
            Ty::Named { name, args } => Ty::Named {
                name: *name,
                args: args.iter().map(|ty| ty.parameterize(generics)).collect(),
            },
            Ty::Tuple(tys) => Ty::Tuple(tys.iter().map(|ty| ty.parameterize(generics)).collect()),
            Ty::Sum(tys) => Ty::Sum(tys.iter().map(|ty| ty.parameterize(generics)).collect()),
            Ty::Function {
                receiver,
                args,
                ret,
            } => {
                let receiver = receiver
                    .as_ref()
                    .map(|r| Box::new(r.parameterize(generics)));
                Ty::Function {
                    receiver,
                    args: args.iter().map(|ty| ty.parameterize(generics)).collect(),
                    ret: Box::new(ret.parameterize(generics)),
                }
            }
            Ty::Meta(_) => unimplemented!("Need to thing about this..."),
            _ => self.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        ty::{Generic, Ty},
        util::Span,
    };

    #[test]
    fn parameterize_generic() {
        let ty = Ty::Generic(Generic::new(("T".to_string(), Span::splat(0))));
        let mut implied = HashMap::new();
        implied.insert("T".to_string(), Ty::int());
        let new_ty = ty.parameterize(&implied);
        assert_eq!(new_ty, Ty::int());
    }

    #[test]
    fn parameterize_named() {
        let ty = Ty::Named {
            name: 0,
            args: vec![Ty::Generic(Generic::new(("T".to_string(), Span::splat(0))))],
        };
        let mut implied = HashMap::new();
        implied.insert("T".to_string(), Ty::int());
        let new_ty = ty.parameterize(&implied);
        assert_eq!(
            new_ty,
            Ty::Named {
                name: 0,
                args: vec![Ty::int()]
            }
        );
    }

    #[test]
    fn parameterize_tuple() {
        let ty = Ty::Tuple(vec![Ty::Generic(Generic::new((
            "T".to_string(),
            Span::splat(0),
        )))]);
        let mut implied = HashMap::new();
        implied.insert("T".to_string(), Ty::int());
        let new_ty = ty.parameterize(&implied);
        assert_eq!(new_ty, Ty::Tuple(vec![Ty::int()]));
    }

    #[test]
    fn parameterize_sum() {
        let ty = Ty::Sum(vec![Ty::Generic(Generic::new((
            "T".to_string(),
            Span::splat(0),
        )))]);
        let mut implied = HashMap::new();
        implied.insert("T".to_string(), Ty::int());
        let new_ty = ty.parameterize(&implied);
        assert_eq!(new_ty, Ty::Sum(vec![Ty::int()]));
    }

    #[test]
    fn parameterize_function() {
        let ty = Ty::Function {
            receiver: Some(Box::new(Ty::Generic(Generic::new((
                "T".to_string(),
                Span::splat(0),
            ))))),
            args: vec![Ty::Generic(Generic::new(("T".to_string(), Span::splat(0))))],
            ret: Box::new(Ty::Generic(Generic::new(("T".to_string(), Span::splat(0))))),
        };
        let mut implied = HashMap::new();
        implied.insert("T".to_string(), Ty::int());
        let new_ty = ty.parameterize(&implied);
        assert_eq!(
            new_ty,
            Ty::Function {
                receiver: Some(Box::new(Ty::int())),
                args: vec![Ty::int()],
                ret: Box::new(Ty::int())
            }
        );
    }

    #[test]
    fn parameterize_type_var() {
        let ty = Ty::TypeVar { id: 0 };
        let mut implied = HashMap::new();
        implied.insert("T".to_string(), Ty::int());
        let new_ty = ty.parameterize(&implied);
        assert_eq!(new_ty, Ty::TypeVar { id: 0 });
    }
}
