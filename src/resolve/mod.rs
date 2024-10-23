use crate::{
    db::{
        input::{Db, SourceFile, Vfs, VfsInner},
        modules::ModulePath,
    },
    parser::{parse_file, top::Top},
    project::{decl::{Decl, DeclKind}, ImplForDecl}, util::Span,
};

use self::state::ResolveState;

mod common;
pub mod state;
mod top;

#[salsa::tracked]
pub fn resolve_file<'db>(db: &'db dyn Db, file: SourceFile) -> Decl<'db> {
    let mut state = ResolveState::from_file(db, file);
    let ast = parse_file(db, file);
    let decls = ast
        .tops(db)
        .iter()
        .filter_map(|item| item.0.resolve(&mut state))
        .collect();
    Decl::new(
        db,
        file.name(db).to_string(),
        Span::splat(0),
        DeclKind::Module(decls),
        Some(file),
        ModulePath::new(db, state.path.clone()),
    )
}

#[salsa::tracked]
pub fn resolve_impls<'db>(db: &'db dyn Db, file: SourceFile) -> Vec<ImplForDecl<'db>> {
    let mut state = ResolveState::from_file(db, file);
    let ast = parse_file(db, file);
    ast.tops(db)
        .iter()
        .filter_map(|item| {
            if let Top::Use(u) = &item.0 {
                state.import(u);
            }
            if let Top::Impl(impl_) = &item.0 {
                state.enter_scope();
                let impl_ = impl_.resolve(&mut state);
                state.exit_scope();
                Some(impl_)
            } else {
                None
            }
        })
        .collect()
}

#[salsa::tracked]
pub fn resolve_vfs<'db>(db: &'db dyn Db, vfs: Vfs, path: ModulePath<'db>) -> Decl<'db> {
    match vfs.inner(db) {
        VfsInner::File(file) => resolve_file(db, *file),
        VfsInner::Dir(files) => {
            let mut modules = Vec::new();
            let mut path = path.name(db).clone();
            for file in files {
                path.push(file.name(db));
                let module = resolve_vfs(db, *file, ModulePath::new(db, path.clone()));
                path.pop();
                modules.push(module);
            }
            Decl::new(
                db,
                vfs.name(db),
                Span::splat(0),
                DeclKind::Module(modules),
                None,
                ModulePath::new(db, path),
            )
        }
    }
}

#[salsa::tracked]
pub fn resolve_impls_vfs<'db>(db: &'db dyn Db, vfs: Vfs) -> Vec<ImplForDecl<'db>> {
    match vfs.inner(db) {
        VfsInner::File(file) => resolve_impls(db, *file),
        VfsInner::Dir(files) => {
            let mut impls = Vec::new();
            for file in files {
                let file_impls = resolve_impls_vfs(db, *file);
                impls.extend(file_impls);
            }
            impls
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
