use crate::{
    db::{
        err::{Diagnostic, Level},
        input::{Db, SourceFile},
    },
    util::Span,
};

use super::IntoWithDb;

#[derive(Clone, Debug, PartialEq)]
pub struct UnboundTypeVar {
    pub span: Span,
    pub file: SourceFile,
    pub name: String,
}

impl UnboundTypeVar {

    pub fn message(&self) -> String {
        format!(
            "Cannot imply type for type generic parameter '{}'",
            self.name
        )
    }
}

impl IntoWithDb<Diagnostic> for UnboundTypeVar {
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
