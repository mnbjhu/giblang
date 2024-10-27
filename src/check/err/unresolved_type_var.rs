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
}

impl UnboundTypeVar {
    pub fn message() -> String {
            "Cannot imply type for type variable".to_string()
    }
}

impl IntoWithDb<Diagnostic> for UnboundTypeVar {
    fn into_with_db(self, db: &dyn Db) -> Diagnostic {
        Diagnostic {
            message: Self::message(),
            span: self.span,
            level: Level::Error,
            path: self.file.path(db),
            file: self.file,
        }
    }
}
