use crate::{
    db::modules::{Module, ModuleData, ModulePath},
    parser::top::enum_::Enum,
    project::decl::{Decl, DeclKind},
    resolve::state::ResolveState,
};

impl Enum {
    pub fn resolve<'db>(&self, state: &mut ResolveState<'db>) -> Decl<'db> {
        let generics = self.generics.0.resolve(state);
        let mut variants = vec![];
        for m in &self.members {
            let decl = m.0.resolve(state);
            variants.push(Module::new(
                state.db,
                decl.name(state.db),
                ModuleData::Export(decl),
            ));
        }
        let kind = DeclKind::Enum { generics, variants };
        Decl::new(state.db, self.name.0.clone(), self.name.1, kind)
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::project::{
//         decl::{struct_::StructDecl, Decl},
//         Project,
//     };
//
//     #[test]
//     fn resolve_enum() {
//         let mut project = Project::new();
//         project.insert_file(
//             "test.gib".to_string(),
//             r"
//             enum Foo {
//                 Bar,
//                 Baz {
//                     x: String,
//                 },
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
//         if let Decl::Enum {
//             name,
//             generics,
//             variants,
//         } = decl
//         {
//             assert_eq!(name.0, "Foo");
//             assert!(generics.is_empty());
//             assert_eq!(variants.len(), 2);
//
//             if let Decl::Member { name, body } = project.get_decl(variants[0]) {
//                 assert_eq!(name.0, "Bar");
//                 assert!(
//                     matches!(body, StructDecl::None),
//                     "Expected Bar to have no body"
//                 );
//             } else {
//                 panic!("Expected Member Decl");
//             }
//             if let Decl::Member { name, body } = project.get_decl(variants[1]) {
//                 assert_eq!(name.0, "Baz");
//                 if let StructDecl::Fields(fields) = body {
//                     assert_eq!(fields.len(), 1);
//                     assert_eq!(fields[0].0, "x");
//                     assert_eq!(fields[0].1, parse_ty(&project, "String"));
//                 } else {
//                     panic!("Expected struct fields");
//                 }
//             } else {
//                 panic!("Expected Member Decl");
//             }
//         } else {
//             panic!("Expected Enum Decl");
//         }
//     }
// }
