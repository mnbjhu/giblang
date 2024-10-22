use std::collections::HashMap;

use crate::{
    db::input::SourceFile,
    ty::{Generic, Ty},
    util::{Span, Spanned},
};

#[derive(Default)]
pub struct TypeState<'db> {
    pub vars: HashMap<u32, MaybeTypeVar<'db>>,
    counter: u32,
}

impl<'db> TypeState<'db> {
    pub fn new_type_var(&mut self, span: Span, file: SourceFile) -> u32 {
        let id = self.counter;
        let new = MaybeTypeVar::Data(TypeVarData::new(span, file));
        self.vars.insert(id, new);
        self.counter += 1;
        id
    }

    pub fn new_type_var_with_bound(
        &mut self,
        generic: Generic<'db>,
        span: Span,
        file: SourceFile,
    ) -> u32 {
        let id = self.counter;
        let new = MaybeTypeVar::Data(TypeVarData {
            bounds: vec![generic],
            explicit: None,
            resolved: None,
            file,
            span,
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

    pub fn get_type_var(&self, id: u32) -> &TypeVarData<'db> {
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

    pub fn get_type_var_mut(&mut self, id: u32) -> &mut TypeVarData<'db> {
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

pub enum MaybeTypeVar<'db> {
    Data(TypeVarData<'db>),
    Pointer(u32),
}

impl<'db> MaybeTypeVar<'db> {
    fn unwrap(self) -> TypeVarData<'db> {
        match self {
            MaybeTypeVar::Data(data) => data,
            MaybeTypeVar::Pointer(_) => panic!("Called unwrap on MaybeTypeVar::Pointer"),
        }
    }
}

pub struct TypeVarData<'db> {
    pub bounds: Vec<Generic<'db>>,
    pub explicit: Option<Spanned<Ty<'db>>>,
    pub resolved: Option<Ty<'db>>,
    pub span: Span,
    pub file: SourceFile,
}

impl<'db> TypeVarData<'db> {
    fn new(span: Span, file: SourceFile) -> TypeVarData<'db> {
        TypeVarData {
            bounds: Vec::default(),
            explicit: None,
            resolved: None,
            span,
            file,
        }
    }
}
