use crate::{
    db::{
        err::{Diagnostic, Level},
        input::{Db, SourceFile},
    },
    util::Span,
};

use super::IntoWithDb;

#[derive(Clone, Debug, PartialEq)]
pub struct UnexpectedArgs {
    pub span: Span,
    pub file: SourceFile,
    pub func: String,
    pub expected: usize,
    pub found: usize,
}

impl UnexpectedArgs {

    pub fn message(&self) -> String {
        format!(
            "Expected {} arguments but found {}",
            self.expected, self.found,
        )
    }
}

impl IntoWithDb<Diagnostic> for UnexpectedArgs {
    fn into_with_db(self, db: &dyn Db) -> Diagnostic {
        Diagnostic {
            message: self.message(),
            span: self.span,
            level: Level::Error,
            path: self.file.path(db),
            file: self.file,
        }
    }
}
