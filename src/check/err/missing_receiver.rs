use crate::{
    db::{
        err::{Diagnostic, Level},
        input::{Db, SourceFile},
    },
    util::Span,
};

use super::IntoWithDb;

#[derive(Clone, Debug, PartialEq)]
pub struct MissingReceiver {
    pub expected: String,
    pub span: Span,
    pub file: SourceFile,
}

impl MissingReceiver {
    pub fn message(&self) -> String {
        format!(
            "Expected function to have a receiver of type {} but found no receiver",
            self.expected,
        )
    }
}

impl IntoWithDb<Diagnostic> for MissingReceiver {
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
