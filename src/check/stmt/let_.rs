use crate::{check::CheckState, parser::stmt::let_::LetStatement, project::Project};

impl LetStatement {
    pub fn check<'module>(
        &'module self,
        project: &'module Project,
        state: &mut CheckState<'module>,
    ) {
        let ty = if let Some(expected) = &self.ty {
            let expected = expected.0.check(project, state);
            self.value
                .0
                .expect_instance_of(&expected, project, state, self.value.1);
            expected
        } else {
            self.value.0.check(project, state)
        };
        self.pattern.0.check(project, state, ty);
    }
}

#[cfg(test)]
mod tests {
    use chumsky::{input::Input, Parser};

    use crate::{
        check::{
            err::{unresolved, CheckError},
            state::{self, CheckState},
        },
        lexer::parser::lexer,
        parser::{
            expr::expr_parser,
            stmt::{let_::let_parser, stmt_parser},
        },
        project::{check_test_state, Project},
        util::Span,
    };

    fn check_let<'project>(
        text: &'static str,
        project: &'project Project,
        state: &mut CheckState<'project>,
    ) -> Vec<CheckError> {
        let tokens = lexer().parse(text).unwrap();
        let input = tokens.spanned(Span::splat(text.len()));
        let parser = let_parser(expr_parser(stmt_parser()));
        let let_ = parser.parse(input).unwrap();
        state.enter_scope();
        // let_.check(project, state);
        state.errors.clone()
    }

    #[test]
    fn test_let() {
        let project = Project::check_test();
        let mut state = check_test_state(&project);
        let errors = check_let("let x = 5", &project, &mut state);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_let_ty() {
        let project = Project::check_test();
        let mut state = check_test_state(&project);
        let errors = check_let("let x: Abc = 5", &project, &mut state);
        assert_eq!(errors.len(), 1);
        let error = errors.get(0).unwrap();
        if let CheckError::Unresolved(unresolved) = error {
            assert_eq!(unresolved.name.0, "Abc");
        } else {
            panic!("Expected Unresolved error");
        }
    }

    #[test]
    fn test_imply_ty() {
        let project = Project::check_test();
        let mut state = check_test_state(&project);
        let errors = check_let("let x = Some(5)", &project, &mut state);
    }
}
