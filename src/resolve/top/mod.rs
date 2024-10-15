use crate::{parser::top::Top, project::decl::Decl};

use super::state::ResolveState;

pub mod enum_;
pub mod enum_member;
pub mod func;
pub mod func_arg;
pub mod impl_;
pub mod struct_;
pub mod struct_body;
pub mod trait_;

impl Top {
    pub fn resolve<'db>(&self, state: &mut ResolveState<'db>) -> Option<Decl<'db>> {
        match self {
            Top::Func(f) => Some(f.resolve(state)),
            Top::Struct(s) => Some(s.resolve(state)),
            Top::Enum(e) => Some(e.resolve(state)),
            Top::Trait(t) => Some(t.resolve(state)),
            Top::Use(u) => {
                state.import(u);
                None
            }
            Top::Impl(_) => None,
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::{check::ty::tests::parse_ty, project::Project};
//
//     #[test]
//     fn resolve_top() {
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
//         let impls = project.get_impls(foo);
//         assert_eq!(impls.len(), 1);
//
//         let resolved_impl = impls[0];
//         assert_eq!(resolved_impl.from, parse_ty(&project, "Foo"));
//         assert_eq!(resolved_impl.to, parse_ty(&project, "Bar"));
//     }
// }
