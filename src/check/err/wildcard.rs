use crate::{
    db::{
        err::{Diagnostic, Level},
        input::{Db, SourceFile},
    },
    util::Span,
};

use super::IntoWithDb;

#[derive(Clone, Debug)]
pub struct UnexpectedWildcard {
    pub span: Span,
    pub file: SourceFile,
}

impl IntoWithDb<Diagnostic> for UnexpectedWildcard {
    fn into_with_db(self, db: &dyn Db) -> Diagnostic {
        Diagnostic {
            message: "Unexpected Wildcard".to_string(),
            span: self.span,
            level: Level::Error,
            path: self.file.path(db),
            file: self.file,
        }
    }
}
