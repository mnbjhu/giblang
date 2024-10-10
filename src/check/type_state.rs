use std::collections::HashMap;

use crate::{
    parser::expr::Expr,
    ty::{Generic, Ty},
    util::{Span, Spanned},
};

#[derive(Default)]
pub struct TypeState<'ty, 'db: 'ty> {
    pub vars: HashMap<u32, MaybeTypeVar<'ty, 'db>>,
    counter: u32,
}

impl<'ty, 'db: 'ty> TypeState<'ty, 'db> {
    pub fn new_type_var(&mut self, span: Span) -> u32 {
        let id = self.counter;
        let new = MaybeTypeVar::Data(TypeVarData::new(span));
        self.vars.insert(id, new);
        self.counter += 1;
        id
    }

    pub fn new_type_var_with_bound(&mut self, generic: Generic<'db>) -> u32 {
        let id = self.counter;
        let new = MaybeTypeVar::Data(TypeVarData {
            span: generic.name.1,
            bounds: vec![generic],
            usages: vec![],
            explicit: None,
            resolved: None,
        });
        self.vars.insert(id, new);
        self.counter += 1;
        id
    }

    fn get_data_pointer(&self, id: u32) -> u32 {
        let mut current = self
            .vars
            .get(&id)
            .expect("All type var ids should be valid");
        let mut ret = id;
        while let MaybeTypeVar::Pointer(id) = current {
            ret = *id;
            current = self.vars.get(id).expect("All type var ids should be valid");
        }
        ret
    }

    pub fn get_type_var(&self, id: u32) -> &TypeVarData {
        let data_pointer = self.get_data_pointer(id);
        let maybe = self
            .vars
            .get(&data_pointer)
            .expect("All type var ids should be valid");
        if let MaybeTypeVar::Data(data) = maybe {
            data
        } else {
            unreachable!()
        }
    }

    fn get_type_var_mut(&mut self, id: u32) -> &mut TypeVarData<'ty, 'db> {
        let data_pointer = self.get_data_pointer(id);
        let maybe = self
            .vars
            .get_mut(&data_pointer)
            .expect("All type var ids should be valid");
        if let MaybeTypeVar::Data(data) = maybe {
            data
        } else {
            unreachable!()
        }
    }

    pub fn add_explicit_type(&mut self, id: u32, ty: Spanned<Ty<'db>>) {
        let var = self.get_type_var_mut(id);
        var.explicit = Some(ty);
    }

    pub fn expected_var_is_ty(&mut self, id: u32, ty: Ty<'db>, span: Span) {
        if let Ty::TypeVar { id: second } = ty {
            self.merge(id, second);
            return;
        }
        let var = self.get_type_var_mut(id);
        var.usages.push(TypeVarUsage::VarIsTy((ty, span)));
    }

    pub fn expected_ty_is_var(&mut self, id: u32, ty: Ty<'db>, span: Span) {
        if let Ty::TypeVar { id: second } = ty {
            self.merge(id, second);
            return;
        }
        let var = self.get_type_var_mut(id);
        var.usages.push(TypeVarUsage::TyIsVar((ty, span)));
    }

    pub fn expect_var_is_expr(&mut self, id: u32, expr: &'ty Spanned<Expr>) {
        let var = self.get_type_var_mut(id);
        var.usages.push(TypeVarUsage::VarIsExpr(expr));
    }

    pub fn expect_expr_is_var(&mut self, id: u32, expr: &'ty Spanned<Expr>) {
        let var = self.get_type_var_mut(id);
        var.usages.push(TypeVarUsage::ExprIsVar(expr));
    }

    pub fn merge(&mut self, first: u32, second: u32) {
        let first = self.get_data_pointer(first);
        let second = self.get_data_pointer(second);
        if first == second {
            return;
        }
        let second = self
            .vars
            .insert(second, MaybeTypeVar::Pointer(first))
            .expect("Expected second type var to exist")
            .unwrap();
        let first = self.get_type_var_mut(first);
        first.usages.extend(second.usages);
        first.bounds.extend(second.bounds);
        let explicit = match (&first.explicit, &second.explicit) {
            (None, None) => None,
            (None, Some(ty)) | (Some(ty), None) => Some(ty.clone()),
            (Some(_), Some(_)) => todo!("Probably a few options here..."),
        };
        first.explicit = explicit;
    }

    pub fn add_bound(&mut self, id: u32, bound: Generic<'db>) {
        let var = self.get_type_var_mut(id);
        var.bounds.push(bound);
    }
}

pub enum MaybeTypeVar<'ty, 'db: 'ty> {
    Data(TypeVarData<'ty, 'db>),
    Pointer(u32),
}

impl<'ty, 'db: 'ty> MaybeTypeVar<'ty, 'db> {
    fn unwrap(self) -> TypeVarData<'ty, 'db> {
        match self {
            MaybeTypeVar::Data(data) => data,
            MaybeTypeVar::Pointer(_) => panic!("Called unwrap on MaybeTypeVar::Pointer"),
        }
    }
}

pub struct TypeVarData<'ty, 'db: 'ty> {
    pub bounds: Vec<Generic<'db>>,
    pub usages: Vec<TypeVarUsage<'ty, 'db>>,
    pub explicit: Option<Spanned<Ty<'db>>>,
    pub resolved: Option<Ty<'db>>,
    pub span: Span,
}

impl<'ty, 'db: 'ty> TypeVarData<'ty, 'db> {
    fn new(span: Span) -> TypeVarData<'ty, 'db> {
        TypeVarData {
            bounds: Vec::default(),
            usages: Vec::default(),
            explicit: None,
            resolved: None,
            span,
        }
    }
}

#[derive(Clone)]
pub enum TypeVarUsage<'ty, 'db: 'ty> {
    VarIsExpr(&'ty Spanned<Expr>),
    ExprIsVar(&'ty Spanned<Expr>),
    VarIsTy(Spanned<Ty<'db>>),
    TyIsVar(Spanned<Ty<'db>>),
}

impl<'ty, 'db: 'ty> TypeVarData<'ty, 'db> {
    pub fn resolve(&mut self) {
        if let Some(ty) = &self.explicit {
            self.resolved = Some(ty.0.clone());
        }
        if let Some(usage) = self.usages.first() {
            self.resolved = match usage {
                TypeVarUsage::VarIsTy(ty) | TypeVarUsage::TyIsVar(ty) => Some(ty.0.clone()),
                _ => todo!("Check if needed"),
            };
        } else {
            self.resolved = Some(Ty::Unknown);
        }
    }
}
