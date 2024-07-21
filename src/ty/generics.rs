use super::{Generic, Ty};

impl Ty {
    pub fn get_generic_params(&self) -> Vec<Generic> {
        match self {
            Ty::Named { args, .. } => args.iter().flat_map(Ty::get_generic_params).collect(),
            Ty::Generic(generic) => vec![generic.clone()],
            Ty::Prim(_) => vec![],
            Ty::Meta(_) => todo!(),
            Ty::Function {
                receiver,
                args,
                ret,
            } => {
                let mut geneircs = vec![];
                if let Some(rec) = receiver {
                    geneircs.extend(rec.get_generic_params())
                }
                geneircs.extend(args.iter().flat_map(Ty::get_generic_params));
                geneircs.extend(ret.get_generic_params());
                geneircs
            }
            Ty::Tuple(v) => v.iter().flat_map(Ty::get_generic_params).collect(),
            Ty::Sum(v) => v.iter().flat_map(Ty::get_generic_params).collect(),
            _ => vec![],
        }
    }
}
