use crate::{
    db::{
        err::{Diagnostic, Level},
        input::{Db, SourceFile},
    },
    util::Spanned,
};

use super::IntoWithDb;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unresolved {
    pub name: Spanned<String>,
    pub file: SourceFile,
}

impl Unresolved {
    pub fn message(&self) -> String {
        format!("Unresolved name `{}`", self.name.0)
    }
}

impl IntoWithDb<Diagnostic> for Unresolved {
    fn into_with_db(self, db: &dyn Db) -> Diagnostic {
        Diagnostic {
            message: self.message(),
            span: self.name.1,
            level: Level::Error,
            path: self.file.path(db),
            file: self.file,
        }
    }
}
