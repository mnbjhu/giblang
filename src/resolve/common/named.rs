use crate::{check::state::CheckState, parser::common::type_::NamedType, ty::Ty};

impl NamedType {
    pub fn resolve(&self, state: &mut CheckState) -> Ty {
        if self.name.len() == 1 {
            if let Some(generic) = state.get_generic(&self.name[0].0) {
                return Ty::Generic(generic.clone());
            }
        };
        if let Some(decl) = state.get_decl_without_error(&self.name) {
            return Ty::Named {
                name: decl,
                args: self.args.iter().map(|ty| ty.0.resolve(state)).collect(),
            };
        }
        Ty::Unknown
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        check::{state::CheckState, ty::tests::assert_no_errors},
        parser::common::type_::NamedType,
        project::Project,
        ty::Ty,
    };

    pub fn resolve_test_project() -> Project {
        Project::from(
            r#"struct Foo
            struct Bar[T]
            struct Baz[T, U]"#,
        )
    }

    #[test]
    fn resolve_project_named_type() {
        let named = NamedType {
            name: vec![
                ("main".to_string(), (0..4).into()),
                ("Foo".to_string(), (6..9).into()),
            ],
            args: vec![],
        };
        let project = resolve_test_project();
        let mut state = CheckState::from_file(project.get_file(0).unwrap(), &project);
        let ty = named.resolve(&mut state);
        assert_no_errors(&state.errors, &project);
        assert_eq!(
            ty,
            Ty::Named {
                name: 7,
                args: vec![]
            }
        );
    }

    #[test]
    fn resolve_local_named_type() {
        let named = NamedType {
            name: vec![("Foo".to_string(), (6..9).into())],
            args: vec![],
        };
        let project = resolve_test_project();
        let mut state = CheckState::from_file(project.get_file(0).unwrap(), &project);
        let ty = named.resolve(&mut state);
        assert_no_errors(&state.errors, &project);
        assert_eq!(
            ty,
            Ty::Named {
                name: 7,
                args: vec![]
            }
        );
    }

    #[test]
    fn fail_resolve_named_type() {
        let named = NamedType {
            name: vec![("Invalid".to_string(), (6..9).into())],
            args: vec![],
        };
        let project = resolve_test_project();
        let mut state = CheckState::from_file(project.get_file(0).unwrap(), &project);
        let ty = named.resolve(&mut state);
        assert_no_errors(&state.errors, &project);
        assert_eq!(ty, Ty::Unknown);
    }

    #[test]
    fn fail_resolve_path_named_type() {
        let named = NamedType {
            name: vec![
                ("main".to_string(), (6..9).into()),
                ("Invalid".to_string(), (6..9).into()),
            ],
            args: vec![],
        };
        let project = resolve_test_project();
        let mut state = CheckState::from_file(project.get_file(0).unwrap(), &project);
        let ty = named.resolve(&mut state);
        assert_no_errors(&state.errors, &project);
        assert_eq!(ty, Ty::Unknown);
    }

    #[test]
    fn resolve_primitive_named_type() {
        let named = NamedType {
            name: vec![("String".to_string(), (0..6).into())],
            args: vec![],
        };
        let project = resolve_test_project();
        let mut state = CheckState::from_file(project.get_file(0).unwrap(), &project);
        let ty = named.resolve(&mut state);
        assert_no_errors(&state.errors, &project);
        assert_eq!(
            ty,
            Ty::Named {
                name: 1,
                args: vec![]
            }
        );
    }
}
