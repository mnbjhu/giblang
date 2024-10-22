use crate::{
    db::{
        err::{Diagnostic, Level},
        input::{Db, SourceFile},
    },
    util::Span,
};

use super::IntoWithDb;

#[derive(Clone, Debug, PartialEq)]
pub struct IsNotInstance {
    pub found: String,
    pub expected: String,
    pub span: Span,
    pub file: SourceFile,
}

impl IsNotInstance {

    pub fn message(&self) -> String {
        format!("Expected {} but found {}", self.expected, self.found)
    }
}

impl IntoWithDb<Diagnostic> for IsNotInstance {
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
