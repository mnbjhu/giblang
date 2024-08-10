use crate::{
    project::{file_data::FileData, Project, TypeVar},
    resolve::state::ResolveState,
    ty::{Generic, Ty},
};

pub struct TypeResolveState<'proj> {
    pub resolve_state: ResolveState<'proj>,
    type_vars: Vec<TypeVar>,
    var_count: u32,
    errors: Vec<TypeResolveError>,
}

pub enum TypeResolveError {}

impl<'proj> TypeResolveState<'proj> {
    pub fn from_file(
        file_data: &'proj FileData,
        project: &'proj Project,
    ) -> TypeResolveState<'proj> {
        let state = ResolveState::from_file(file_data, project);
        TypeResolveState {
            resolve_state: state,
            type_vars: Vec::new(),
            var_count: 0,
            errors: Vec::new(),
        }
    }
    pub fn error(&mut self, error: TypeResolveError) {
        self.errors.push(error);
    }

    pub fn imply_type_bound(&mut self, id: u32, ty: Ty) {
        if let Some(var) = self.type_vars.get_mut(id as usize) {
            if var.ty.is_none() {
                var.ty = Some(ty);
            }
        } else {
            panic!("Failed to find type var with id {id}");
        }
    }

    pub fn expect_type_bound(&mut self, id: u32, ty: Ty) {
        if let Some(vars) = self.type_vars.get_mut(id as usize) {
            vars.ty = Some(ty);
        } else {
            panic!("Failed to find type var with id {id}");
        }
    }

    pub fn instantiate(&mut self, generic: Generic) -> u32 {
        let id = self.var_count;
        self.var_count += 1;
        self.type_vars.push(TypeVar {
            id,
            generic,
            ty: None,
        });
        id
    }

    pub fn new_type_var(&mut self) -> u32 {
        let id = self.var_count;
        self.var_count += 1;
        self.type_vars.push(TypeVar {
            id,
            generic: Generic::default(),
            ty: None,
        });
        id
    }

    pub fn imply_is(&mut self, first: &Ty, second: &Ty) {
        todo!()
    }
    pub fn expect_is(&mut self, first: &Ty, second: &Ty) {
        todo!()
    }
}
