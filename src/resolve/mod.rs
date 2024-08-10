use std::collections::HashMap;

use crate::{
    check::{err::CheckError},
    project::{decl::Decl, file_data::FileData, ImplData, Project},
};

use self::state::ResolveState;

mod common;
pub mod state;
mod top;

pub fn resolve_file(
    file_data: &FileData,
    decls: &mut HashMap<u32, Decl>,
    impls: &mut HashMap<u32, ImplData>,
    impl_map: &mut HashMap<u32, Vec<u32>>,
    project: &Project,
) -> Vec<CheckError> {
    let mut state = ResolveState::from_file(file_data, project);
    for (item, _) in &file_data.ast {
        state.enter_scope();
        item.resolve(&mut state, decls, impls, impl_map);
        state.exit_scope();
    }
    state.errors
}

#[cfg(test)]
mod tests {
    use crate::{
        check::ty::tests::parse_ty,
        project::{
            decl::{struct_::StructDecl, Decl},
            Project,
        },
    };

    #[test]
    fn single_file() {
        let mut project = Project::new();
        project.insert_file(
            "test.gib".to_string(),
            r"
            struct Foo {
                x: i32,
            }
            fn main() {
                let x = 5
            }
            "
            .to_string(),
        );

        let errors = project.resolve();
        assert!(errors.is_empty());

        let main = project.get_path(&["test", "main"]);
        if let Some(main) = main {
            let main = project.get_decl(main);
            assert_eq!(main.name(), "main");
        } else {
            panic!("Failed to resolve main");
        }
    }

    #[test]
    fn multi_file() {
        let mut project = Project::new();
        project.insert_file(
            "test.gib".to_string(),
            r"
            struct Foo {
                x: i32,
            }
            fn main() {
                let x = 5
            }
            "
            .to_string(),
        );
        project.insert_file(
            "test2.gib".to_string(),
            r"
            struct Bar {
                y: i32,
            }
            "
            .to_string(),
        );

        let errors = project.resolve();
        assert!(errors.is_empty());

        let main = project.get_path(&["test", "main"]);
        if let Some(main) = main {
            let main = project.get_decl(main);
            assert_eq!(main.name(), "main");
        } else {
            panic!("Failed to resolve main");
        }

        let bar = project.get_path(&["test2", "Bar"]);
        if let Some(bar) = bar {
            let bar = project.get_decl(bar);
            assert_eq!(bar.name(), "Bar");
        } else {
            panic!("Failed to resolve Bar");
        }
    }

    #[allow(clippy::similar_names)]
    #[test]
    fn enum_members() {
        let mut project = Project::new();
        project.insert_file(
            "test.gib".to_string(),
            r"
            enum Foo {
                Bar,
                Baz(Int),
            }
            "
            .to_string(),
        );

        let errors = project.resolve();
        assert!(errors.is_empty());

        let bar = project
            .get_path(&["test", "Foo", "Bar"])
            .expect("Couldn't resolve 'Bar'");
        if let Decl::Member { name, body } = project.get_decl(bar) {
            assert_eq!(name.0, "Bar");
            assert!(body.is_none(), "Expected an 'None' body");
        }

        let baz = project
            .get_path(&["test", "Foo", "Baz"])
            .expect("Couldn't resolve 'Baz'");

        if let Decl::Member { name, body } = project.get_decl(baz) {
            assert_eq!(name.0, "Baz");
            if let StructDecl::Tuple(v) = body {
                assert_eq!(v.len(), 1);
                assert_eq!(v[0], parse_ty(&project, "Int"));
            }
        }
    }
}
