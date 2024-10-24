use crate::{
    db::{
        err::{Diagnostic, Level},
        input::{Db, SourceFile},
    },
    util::Span,
};

use super::IntoWithDb;

#[derive(Clone, Debug)]
pub struct ImplTypeMismatch {
    pub found: String,
    pub span: Span,
    pub file: SourceFile,
}

impl ImplTypeMismatch {
    pub fn message(&self) -> String {
        format!(
            "Expected type to be a named type but found `{}`",
            self.found
        )
    }
}

impl IntoWithDb<Diagnostic> for ImplTypeMismatch {
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
