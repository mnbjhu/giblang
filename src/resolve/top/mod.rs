use std::collections::HashMap;

use crate::{
    check::err::{impl_type::ImplTypeMismatch, ResolveError},
    db::input::Db,
    parser::top::Top,
    project::{decl::Decl, ImplData},
    ty::Ty,
};

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
    pub fn resolve(
        &self,
        db: &dyn Db,
        state: &mut ResolveState,
        decls: &mut HashMap<u32, Decl>,
        impls: &mut HashMap<u32, ImplData>,
        impl_map: &mut HashMap<u32, Vec<u32>>,
    ) {
        if let Top::Use(use_) = self {
            state.import(db, use_);
        } else {
            let id = self.get_id().unwrap();
            let decl = match self {
                Top::Func(f) => f.resolve(state),
                Top::Struct(s) => s.resolve(state),
                Top::Enum(e) => e.resolve(state, decls),
                Top::Trait(t) => t.resolve(state, decls),
                Top::Impl(i) => {
                    let id = i.id;
                    let resolved = i.resolve(state, decls);
                    if let Ty::Named { name, .. } = &resolved.from {
                        if let Some(existing) = impl_map.get_mut(name) {
                            existing.push(id);
                        } else {
                            impl_map.insert(*name, vec![id]);
                        }
                    } else {
                        state.error(ResolveError::ImplTypeMismatch(ImplTypeMismatch {
                            found: resolved.from.kind(),
                            file: state.get_file(),
                            span: i.trait_.1,
                        }));
                    };
                    impls.insert(id, resolved);
                    return;
                }
                Top::Use(_) => todo!(),
            };
            decls.insert(id, decl);
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
