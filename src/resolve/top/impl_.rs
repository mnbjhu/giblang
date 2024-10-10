use crate::{parser::top::impl_::Impl, project::ImplDecl, resolve::state::ResolveState};

impl Impl {
    pub fn resolve<'db>(&self, state: &mut ResolveState<'db>) -> ImplDecl<'db> {
        let generics = self.generics.resolve(state);
        let to = self.trait_.0.resolve(state);
        let from = self.for_.0.resolve(state);
        state.add_self_ty(from.clone(), self.for_.1);
        let mut functions = Vec::new();
        for func in &self.body {
            state.enter_scope();
            let decl = func.0.resolve(state);
            functions.push(decl);
            state.exit_scope();
        }
        ImplDecl::new(state.db, generics, from, to, functions)
    }
}
// #[cfg(test)]
// mod tests {
//     use crate::{
//         check::ty::tests::parse_ty,
//         parser::common::variance::Variance,
//         project::{decl::Decl, Project},
//         ty::Ty,
//     };
//
//     #[test]
//     fn resolve_impl() {
//         let mut project = Project::new();
//         project.insert_file(
//             "test.gib".to_string(),
//             r"
//             struct Foo {
//                 x: i32,
//             }
//             trait Bar
//             impl Bar for Foo
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
//         let bar = project
//             .get_path(&["test", "Bar"])
//             .expect("Failed to resolve Bar");
//         let impls = project.get_impls(foo);
//         assert_eq!(impls.len(), 1);
//
//         let resolved_impl = impls[0];
//
//         if let Ty::Named { name, args } = &resolved_impl.from {
//             assert_eq!(*name, foo);
//             assert_eq!(args.len(), 0);
//         } else {
//             panic!("Expected Named type");
//         }
//
//         if let Ty::Named { name, args } = &resolved_impl.to {
//             assert_eq!(*name, bar);
//             assert_eq!(args.len(), 0);
//         } else {
//             panic!("Expected Named type");
//         }
//     }
//
//     #[test]
//     fn resolve_impl_functions() {
//         let mut project = Project::new();
//         project.insert_file(
//             "test.gib".to_string(),
//             r"
//             struct Foo
//             trait Bar
//             impl Bar for Foo {
//                 fn Self.baz(): Int {
//                     123
//                 }
//                 fn baz(text: String): Int
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
//         let bar = project
//             .get_path(&["test", "Bar"])
//             .expect("Failed to resolve Bar");
//         let impls = project.get_impls(foo);
//         assert_eq!(impls.len(), 1);
//
//         let resolved_impl = impls[0];
//
//         if let Ty::Named { name, args } = &resolved_impl.from {
//             assert_eq!(*name, foo);
//             assert_eq!(args.len(), 0);
//         } else {
//             panic!("Expected Named type");
//         }
//
//         if let Ty::Named { name, args } = &resolved_impl.to {
//             assert_eq!(*name, bar);
//             assert_eq!(args.len(), 0);
//         } else {
//             panic!("Expected Named type");
//         }
//
//         assert_eq!(resolved_impl.functions.len(), 2);
//
//         if let Decl::Function {
//             name,
//             generics,
//             receiver,
//             args,
//             ret,
//         } = project.get_decl(resolved_impl.functions[0])
//         {
//             assert_eq!(name.0, "baz");
//             assert_eq!(generics.len(), 0);
//
//             if let Some(Ty::Generic(rec)) = &receiver {
//                 assert_eq!(rec.name.0, "Self");
//                 assert_eq!(rec.variance, Variance::Invariant);
//                 if let Ty::Named { name, args } = rec.super_.as_ref() {
//                     assert_eq!(*name, foo,);
//                     assert_eq!(args.len(), 0);
//                 }
//             } else {
//                 panic!("Expected generic receiver, buf found {receiver:?}");
//             }
//             assert_eq!(args.len(), 0);
//             assert_eq!(*ret, parse_ty(&project, "Int"));
//         } else {
//             panic!("Expected Function decl");
//         }
//
//         if let Decl::Function {
//             name,
//             generics,
//             receiver,
//             args,
//             ret,
//         } = project.get_decl(resolved_impl.functions[1])
//         {
//             assert_eq!(name.0, "baz");
//             assert_eq!(generics.len(), 0);
//
//             assert!(receiver.is_none());
//
//             assert_eq!(args.len(), 1);
//             assert_eq!(args[0].0, "text");
//             assert_eq!(args[0].1, parse_ty(&project, "String"));
//             assert_eq!(*ret, parse_ty(&project, "Int"));
//         } else {
//             panic!("Expected Function decl");
//         }
//     }
// }
