use crate::{
    db::{
        input::{Db, SourceFile, Vfs, VfsInner},
        modules::{Module, ModuleData},
    },
    parser::parse_file,
};
use tracing::info;

use self::state::ResolveState;

mod common;
pub mod state;
mod top;

#[salsa::tracked]
pub fn resolve_file<'db>(db: &'db dyn Db, file: SourceFile) -> Module<'db> {
    info!("Resolving file {}", file.name(db));
    let mut state = ResolveState::from_file(db, file);
    let decls = parse_file(db, file)
        .tops(db)
        .iter()
        .filter_map(|item| {
            state.enter_scope();
            let found = item.data(db).resolve(&mut state);
            state.exit_scope();
            found
        })
        .map(|decl| {
            let name = decl.name(db);
            let export = ModuleData::Export(decl);
            Module::new(db, name, export)
        })
        .collect();
    Module::new(db, file.name(db).to_string(), ModuleData::Package(decls))
}

#[salsa::tracked]
pub fn resolve_vfs<'db>(db: &'db dyn Db, vfs: Vfs) -> Module<'db> {
    info!("Resolving VFS {}", vfs.name(db));
    match vfs.inner(db) {
        VfsInner::File(file) => resolve_file(db, *file),
        VfsInner::Dir(files) => {
            let mut modules = Vec::new();
            for file in files {
                let module = resolve_vfs(db, *file);
                modules.push(module);
            }
            Module::new(db, "root".to_string(), ModuleData::Package(modules))
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::{
//         check::ty::tests::parse_ty,
//         project::{
//             decl::{struct_::StructDecl, Decl},
//             Project,
//         },
//     };
//
//     #[test]
//     fn single_file() {
//         let mut project = Project::new();
//         project.insert_file(
//             "test.gib".to_string(),
//             r"
//             struct Foo {
//                 x: i32,
//             }
//             fn main() {
//                 let x = 5
//             }
//             "
//             .to_string(),
//         );
//
//         let errors = project.resolve();
//         assert!(errors.is_empty());
//
//         let main = project.get_path(&["test", "main"]);
//         if let Some(main) = main {
//             let main = project.get_decl(main);
//             assert_eq!(main.name(), "main");
//         } else {
//             panic!("Failed to resolve main");
//         }
//     }
//
//     #[test]
//     fn multi_file() {
//         let mut project = Project::new();
//         project.insert_file(
//             "test.gib".to_string(),
//             r"
//             struct Foo {
//                 x: i32,
//             }
//             fn main() {
//                 let x = 5
//             }
//             "
//             .to_string(),
//         );
//         project.insert_file(
//             "test2.gib".to_string(),
//             r"
//             struct Bar {
//                 y: i32,
//             }
//             "
//             .to_string(),
//         );
//
//         let errors = project.resolve();
//         assert!(errors.is_empty());
//
//         let main = project.get_path(&["test", "main"]);
//         if let Some(main) = main {
//             let main = project.get_decl(main);
//             assert_eq!(main.name(), "main");
//         } else {
//             panic!("Failed to resolve main");
//         }
//
//         let bar = project.get_path(&["test2", "Bar"]);
//         if let Some(bar) = bar {
//             let bar = project.get_decl(bar);
//             assert_eq!(bar.name(), "Bar");
//         } else {
//             panic!("Failed to resolve Bar");
//         }
//     }
//
//     #[allow(clippy::similar_names)]
//     #[test]
//     fn enum_members() {
//         let mut project = Project::new();
//         project.insert_file(
//             "test.gib".to_string(),
//             r"
//             enum Foo {
//                 Bar,
//                 Baz(Int),
//             }
//             "
//             .to_string(),
//         );
//
//         let errors = project.resolve();
//         assert!(errors.is_empty());
//
//         let bar = project
//             .get_path(&["test", "Foo", "Bar"])
//             .expect("Couldn't resolve 'Bar'");
//         if let Decl::Member { name, body } = project.get_decl(bar) {
//             assert_eq!(name.0, "Bar");
//             assert!(body.is_none(), "Expected an 'None' body");
//         }
//
//         let baz = project
//             .get_path(&["test", "Foo", "Baz"])
//             .expect("Couldn't resolve 'Baz'");
//
//         if let Decl::Member { name, body } = project.get_decl(baz) {
//             assert_eq!(name.0, "Baz");
//             if let StructDecl::Tuple(v) = body {
//                 assert_eq!(v.len(), 1);
//                 assert_eq!(v[0], parse_ty(&project, "Int"));
//             }
//         }
//     }
// }
