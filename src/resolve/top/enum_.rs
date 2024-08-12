use std::collections::HashMap;

use crate::{parser::top::enum_::Enum, project::decl::Decl, resolve::state::ResolveState};

impl Enum {
    pub fn resolve(&self, state: &mut ResolveState, decls: &mut HashMap<u32, Decl>) -> Decl {
        let generics = self.generics.0.resolve(state);
        let mut variants = vec![];
        for m in &self.members {
            let id = m.0.id;
            let decl = m.0.resolve(state);
            decls.insert(id, decl);
            variants.push(id);
        }
        Decl::Enum {
            name: self.name.clone(),
            generics,
            variants,
        }
    }
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
    fn resolve_enum() {
        let mut project = Project::new();
        project.insert_file(
            "test.gib".to_string(),
            r"
            enum Foo {
                Bar,
                Baz {
                    x: String,
                },
            }
            "
            .to_string(),
        );

        let errors = project.resolve();
        assert!(errors.is_empty());

        let foo = project
            .get_path(&["test", "Foo"])
            .expect("Failed to resolve Foo");

        let decl = project.get_decl(foo);

        if let Decl::Enum {
            name,
            generics,
            variants,
        } = decl
        {
            assert_eq!(name.0, "Foo");
            assert!(generics.is_empty());
            assert_eq!(variants.len(), 2);

            if let Decl::Member { name, body } = project.get_decl(variants[0]) {
                assert_eq!(name.0, "Bar");
                assert!(
                    matches!(body, StructDecl::None),
                    "Expected Bar to have no body"
                );
            } else {
                panic!("Expected Member Decl");
            }
            if let Decl::Member { name, body } = project.get_decl(variants[1]) {
                assert_eq!(name.0, "Baz");
                if let StructDecl::Fields(fields) = body {
                    assert_eq!(fields.len(), 1);
                    assert_eq!(fields[0].0, "x");
                    assert_eq!(fields[0].1, parse_ty(&project, "String"));
                } else {
                    panic!("Expected struct fields");
                }
            } else {
                panic!("Expected Member Decl");
            }
        } else {
            panic!("Expected Enum Decl");
        }
    }
}
