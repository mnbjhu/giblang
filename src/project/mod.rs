use std::collections::HashMap;

use decl::Decl;

use crate::{
    db::{
        input::Db,
        modules::{Module, ModuleData, ModulePath},
    },
    ty::{Generic, Ty},
};

pub mod decl;
pub mod file_data;
pub mod inst;
mod module;
pub mod name;
pub mod util;

#[salsa::tracked]
pub struct Project<'db> {
    pub decls: Module<'db>,
    pub impls: HashMap<ModulePath<'db>, ImplDecl<'db>>,
    pub impl_map: HashMap<ImplDecl<'db>, Vec<ImplDecl<'db>>>,
}

#[salsa::tracked]
pub struct ImplDecl<'db> {
    pub generics: Vec<Generic<'db>>,
    #[id]
    pub from_ty: Ty<'db>,
    #[id]
    pub to_ty: Ty<'db>,
    pub functions: Vec<Decl<'db>>,
}

// #[cfg(test)]
// #[must_use]
// pub fn check_test_state(project: &Project) -> CheckState {
//     CheckState::from_file(project.get_file(0).unwrap(), project)
// }
//

impl<'db> Project<'db> {
    pub fn get_decl(self, db: &'db dyn Db, path: ModulePath<'db>) -> Option<Decl<'db>> {
        let module = self.decls(db).get_path(db, path)?;
        match module.content(db) {
            ModuleData::Export(decl) => Some(*decl),
            _ => None,
        }
    }

    pub fn get_impl(self, db: &'db dyn Db, path: ModulePath<'db>) -> ImplDecl<'db> {
        *self.impls(db).get(&path).unwrap()
    }

    pub fn get_impls(self, db: &'db dyn Db, path: ModulePath<'db>) -> Vec<ImplDecl<'db>> {
        let impl_ = self.get_impl(db, path);
        self.impl_map(db).get(&impl_).unwrap().clone()
    }
}
#[derive(Debug, Clone)]
pub struct TypeVar<'db> {
    pub id: ModulePath<'db>,
    pub generic: Generic<'db>,
    pub ty: Option<Ty<'db>>,
}

// #[cfg(test)]
// mod tests {
//     use super::Project;
//
//     impl Project {
//         #[must_use]
//         pub fn from(text: &str) -> Project {
//             let mut project = Project::new();
//             project.insert_file("main.gib".to_string(), text.to_string());
//             project
//         }
//
//         #[must_use]
//         pub fn check_test() -> Project {
//             let mut project = Project::from(
//                 r"struct Foo
// struct Bar[T]
// struct Baz[T, U]
// enum Option[out T] {
//    Some(T),
//    None
// }
// enum Result[out R, out E] {
//    Ok(R),
//    Err(E),
// }
// fn add(a: Int, b: Int): Int { }
// fn Int.factorial(): Int { }
// fn ident[T](t: T): T { }
// ",
//             );
//             project.resolve();
//             project
//         }
//     }
// }
