use std::collections::HashMap;

use crate::{
    parser::expr::Expr,
    ty::{Generic, Ty},
    util::{Span, Spanned},
};

use super::{err::CheckError, state::CheckState};

#[derive(Default)]
pub struct TypeState<'ty> {
    pub vars: HashMap<u32, MaybeTypeVar<'ty>>,
    counter: u32,
}

impl<'ast> TypeState<'ast> {
    pub fn new_type_var(&mut self, span: Span) -> u32 {
        let id = self.counter;
        let new = MaybeTypeVar::Data(TypeVarData::new(span));
        self.vars.insert(id, new);
        self.counter += 1;
        id
    }

    pub fn new_type_var_with_bound(&mut self, generic: Generic) -> u32 {
        let id = self.counter;
        let new = MaybeTypeVar::Data(TypeVarData {
            span: generic.name.1,
            bounds: vec![generic],
            usages: vec![],
            explicit: None,
            resolved: Ty::Unknown,
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

    fn get_type_var_mut(&mut self, id: u32) -> &mut TypeVarData<'ast> {
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

    pub fn add_explicit_type(&mut self, id: u32, ty: Spanned<Ty>) {
        let var = self.get_type_var_mut(id);
        var.explicit = Some(ty);
    }

    pub fn expected_var_is_ty(&mut self, id: u32, ty: Ty, span: Span) {
        if let Ty::TypeVar { id: second } = ty {
            self.merge(id, second);
            return;
        }
        let var = self.get_type_var_mut(id);
        var.usages.push(TypeVarUsage::VarIsTy((ty, span)));
    }

    pub fn expected_ty_is_var(&mut self, id: u32, ty: Ty, span: Span) {
        if let Ty::TypeVar { id: second } = ty {
            self.merge(id, second);
            return;
        }
        let var = self.get_type_var_mut(id);
        var.usages.push(TypeVarUsage::TyIsVar((ty, span)));
    }

    pub fn expect_var_is_expr(&mut self, id: u32, expr: &'ast Spanned<Expr>) {
        let var = self.get_type_var_mut(id);
        var.usages.push(TypeVarUsage::VarIsExpr(expr));
    }

    pub fn expect_expr_is_var(&mut self, id: u32, expr: &'ast Spanned<Expr>) {
        let var = self.get_type_var_mut(id);
        var.usages.push(TypeVarUsage::ExprIsVar(expr));
    }

    pub fn merge(&mut self, first: u32, second: u32) {
        println!("Merging ids {first} and {second}");
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
}

pub enum MaybeTypeVar<'ast> {
    Data(TypeVarData<'ast>),
    Pointer(u32),
}

impl<'ast> MaybeTypeVar<'ast> {
    fn unwrap(self) -> TypeVarData<'ast> {
        match self {
            MaybeTypeVar::Data(data) => data,
            MaybeTypeVar::Pointer(_) => panic!("Called unwrap on MaybeTypeVar::Pointer"),
        }
    }
}

pub struct TypeVarData<'ast> {
    pub bounds: Vec<Generic>,
    pub usages: Vec<TypeVarUsage<'ast>>,
    pub explicit: Option<Spanned<Ty>>,
    pub resolved: Ty,
    pub span: Span,
}

impl<'ast> TypeVarData<'ast> {
    fn new(span: Span) -> TypeVarData<'ast> {
        TypeVarData {
            bounds: Default::default(),
            usages: Default::default(),
            explicit: Default::default(),
            resolved: Default::default(),
            span,
        }
    }
}

#[derive(Clone)]
pub enum TypeVarUsage<'ast> {
    VarIsExpr(&'ast Spanned<Expr>),
    ExprIsVar(&'ast Spanned<Expr>),
    VarIsTy(Spanned<Ty>),
    TyIsVar(Spanned<Ty>),
}

impl<'ast> TypeVarData<'ast> {
    pub fn resolve(&mut self) {
        if let Some(ty) = &self.explicit {
            self.resolved = ty.0.clone()
        }
        if let Some(usage) = self.usages.first() {
            self.resolved = match usage {
                TypeVarUsage::VarIsTy(ty) => ty.0.clone(),
                TypeVarUsage::TyIsVar(ty) => ty.0.clone(),
                _ => todo!("Check if needed"),
            };
        }
    }
}
