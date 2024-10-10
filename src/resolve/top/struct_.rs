use crate::{parser::top::struct_::Struct, project::decl::DeclKind, resolve::state::ResolveState};

use super::Decl;

impl Struct {
    pub fn resolve<'db>(&self, state: &mut ResolveState<'db>) -> Decl<'db> {
        let generics = self.generics.0.resolve(state);
        let name = self.name.clone();
        let body = self.body.resolve(state);
        let kind = DeclKind::Struct { generics, body };
        Decl::new(state.db, name.0, name.1, kind)
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::{
//         check::ty::tests::parse_ty,
//         parser::common::variance::Variance,
//         project::{
//             decl::{struct_::StructDecl, Decl},
//             Project,
//         },
//         ty::Ty,
//     };
//
//     #[test]
//     fn resolve_struct() {
//         let mut project = Project::new();
//         project.insert_file(
//             "test.gib".to_string(),
//             r"
//             struct Foo {
//                 x: String,
//             }
//             "
//             .to_string(),
//         );
//
//         let errors = project.resolve();
//         assert!(errors.is_empty());
//
//         let foo = project
//             .get_path(&["test", "Foo"])
//             .expect("Failed to resolve Foo");
//
//         let decl = project.get_decl(foo);
//
//         if let Decl::Struct {
//             name,
//             generics,
//             body,
//         } = decl
//         {
//             assert_eq!(name.0, "Foo");
//             assert!(generics.is_empty());
//             if let StructDecl::Fields(fields) = body {
//                 assert_eq!(fields.len(), 1);
//                 assert_eq!(fields[0].0, "x");
//                 assert_eq!(fields[0].1, parse_ty(&project, "String"));
//             } else {
//                 panic!("Expected struct fields");
//             }
//         } else {
//             panic!("Expected struct declaration");
//         }
//     }
//
//     #[test]
//     fn resolve_struct_with_args() {
//         let mut project = Project::new();
//         project.insert_file(
//             "test.gib".to_string(),
//             r"
//             trait Bar
//             struct Foo[T, out U: Bar] {
//                 x: String,
//             }
//             "
//             .to_string(),
//         );
//
//         let errors = project.resolve();
//         assert!(errors.is_empty());
//
//         let foo = project
//             .get_path(&["test", "Foo"])
//             .expect("Failed to resolve Foo");
//
//         let decl = project.get_decl(foo);
//
//         if let Decl::Struct {
//             name,
//             generics,
//             body,
//         } = decl
//         {
//             assert_eq!(name.0, "Foo");
//             assert_eq!(generics.len(), 2);
//
//             let first_arg = &generics[0];
//             assert_eq!(first_arg.name.0, "T");
//             assert_eq!(first_arg.super_.as_ref(), &Ty::Any);
//             assert_eq!(first_arg.variance, Variance::Invariant);
//
//             let second_arg = &generics[1];
//             assert_eq!(second_arg.name.0, "U");
//             assert_eq!(second_arg.super_.as_ref(), &parse_ty(&project, "Bar"));
//             assert_eq!(second_arg.variance, Variance::Covariant);
//
//             if let StructDecl::Fields(fields) = body {
//                 assert_eq!(fields.len(), 1);
//                 assert_eq!(fields[0].0, "x");
//                 assert_eq!(fields[0].1, parse_ty(&project, "String"));
//             } else {
//                 panic!("Expected struct fields");
//             }
//         } else {
//             panic!("Expected struct declaration");
//         }
//     }
// }
