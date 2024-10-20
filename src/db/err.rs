use std::path::PathBuf;

use crate::{lexer::keyword::Keyword, util::Span};

use super::input::SourceFile;

#[salsa::accumulator]
pub struct Diagnostic {
    pub message: String,
    pub span: Span,
    pub level: Level,
    pub path: PathBuf,
    pub file: SourceFile,
}

#[derive(Clone, Debug)]
pub enum Level {
    Error,
    Warning,
}
