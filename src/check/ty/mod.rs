use crate::{check::state::CheckState, parser::common::type_::Type, project::Project, ty::Ty};
pub mod named;

impl Type {
    pub fn check(&self, project: &Project, state: &mut CheckState) -> Ty {
        match &self {
            Type::Named(named) => named.check(state, project),
            Type::Tuple(tup) => {
                let mut tys = vec![];
                for (ty, _) in tup {
                    tys.push(ty.check(project, state))
                }
                Ty::Tuple(tys)
            }
            Type::Sum(tup) => {
                let mut tys = vec![];
                for (ty, _) in tup {
                    tys.push(ty.check(project, state))
                }
                Ty::Sum(tys)
            }
            Type::Function {
                receiver,
                args,
                ret,
            } => Ty::Function {
                receiver: receiver
                    .as_ref()
                    .map(|receiver| Box::new(receiver.as_ref().0.check(project, state))),
                args: args.iter().map(|r| r.0.check(project, state)).collect(),
                ret: Box::new(ret.0.check(project, state)),
            },
        }
    }
}

#[cfg(test)]
pub mod tests {
    use chumsky::{input::Input, Parser};

    use crate::{
        check::{
            err::{unresolved::Unresolved, CheckError},
            state::CheckState,
        },
        lexer::parser::lexer,
        parser::common::type_::type_parser,
        project::Project,
        util::Span,
    };

    use super::Ty;

    pub fn try_parse_ty(project: &Project, ty: &str) -> (Ty, Vec<CheckError>) {
        let eoi = Span::splat(ty.len());
        let tokens = lexer().parse(ty).unwrap();
        let ty = type_parser().parse(tokens.spanned(eoi)).unwrap();
        let file_data = project.get_file(project.get_counter()).unwrap();
        let mut state = CheckState::from_file(file_data, project);
        (ty.check(project, &mut state), state.errors)
    }

    pub fn parse_ty(project: &Project, ty: &str) -> Ty {
        let eoi = Span::splat(ty.len());
        let tokens = lexer().parse(ty).unwrap();
        let ty = type_parser().parse(tokens.spanned(eoi)).unwrap();
        let file_data = project.get_file(project.get_counter()).unwrap();
        let mut state = CheckState::from_file(file_data, project);
        assert_no_errors(&state.errors, project);
        ty.check(project, &mut state)
    }

    pub fn assert_no_errors(errors: &Vec<CheckError>, project: &Project) {
        if errors.is_empty() {
            return;
        }
        for error in errors {
            project.print_error(error);
        }
        panic!("Expected there to be no 'check' errors")
    }

    #[test]
    fn check_unit() {
        let project = Project::check_test();
        let (unit, err) = try_parse_ty(&project, "()");
        assert_no_errors(&err, &project);

        if let Ty::Tuple(tys) = unit {
            assert_eq!(tys.len(), 0);
        } else {
            panic!("Expected unit type to be a tuple")
        }
    }

    #[test]
    fn check_string() {
        let project = Project::check_test();
        let (string, err) = try_parse_ty(&project, "String");
        assert_no_errors(&err, &project);

        if let Ty::Named { name, args } = string {
            assert_eq!(name, 1);
            assert_eq!(args.len(), 0);
        } else {
            panic!("Expected string type to be a named type")
        }
    }

    #[test]
    fn check_foo() {
        let project = Project::check_test();
        let (foo, err) = try_parse_ty(&project, "Foo");
        assert_no_errors(&err, &project);

        if let Ty::Named { name, args } = foo {
            assert_eq!(project.get_decl(name).name(), "Foo");
            assert_eq!(args.len(), 0);
        } else {
            panic!("Expected foo type to be a named type")
        }
    }

    #[test]
    fn check_bar() {
        let project = Project::check_test();
        let (bar, err) = try_parse_ty(&project, "Bar[Foo]");
        assert_no_errors(&err, &project);
        if let Ty::Named { name, args } = bar {
            assert_eq!(project.get_decl(name).name(), "Bar");
            assert_eq!(args.len(), 1);
            if let Ty::Named { name, args } = &args[0] {
                assert_eq!(project.get_decl(*name).name(), "Foo");
                assert_eq!(args.len(), 0);
            } else {
                panic!("Expected bar type to have a single argument")
            }
        } else {
            panic!("Expected bar type to be a named type")
        }
    }

    #[test]
    fn check_tuple() {
        let project = Project::check_test();
        let (tuple, err) = try_parse_ty(&project, "(Foo, Bar[Foo])");
        assert_no_errors(&err, &project);

        if let Ty::Tuple(tys) = tuple {
            assert_eq!(tys.len(), 2);
            if let Ty::Named { name, args } = &tys[0] {
                assert_eq!(project.get_decl(*name).name(), "Foo");
                assert_eq!(args.len(), 0);
            } else {
                panic!("Expected first element of tuple to be a named type")
            }
            if let Ty::Named { name, args } = &tys[1] {
                assert_eq!(project.get_decl(*name).name(), "Bar");
                assert_eq!(args.len(), 1);
                if let Ty::Named { name, args } = &args[0] {
                    assert_eq!(project.get_decl(*name).name(), "Foo");
                    assert_eq!(args.len(), 0);
                } else {
                    panic!("Expected second element of tuple to be a named type")
                }
            } else {
                panic!("Expected second element of tuple to be a named type")
            }
        } else {
            panic!("Expected tuple type to be a tuple")
        }
    }

    #[test]
    fn check_unresolved() {
        let project = Project::check_test();
        let (unresolved, err) = try_parse_ty(&project, "Unresolved");
        assert_eq!(err.len(), 1);
        if let CheckError::Unresolved(err) = &err[0] {
            assert_eq!(err.name.0, "Unresolved");
        } else {
            panic!("Expected unresolved error")
        }
        assert_eq!(unresolved, Ty::Unknown)
    }
}
