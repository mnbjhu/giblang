use std::path::PathBuf;

use crate::util::Span;

use super::input::SourceFile;

#[salsa::accumulator]
pub struct Diagnostic {
    pub message: String,
    pub span: Span,
    #[allow(dead_code)]
    pub level: Level,
    pub path: PathBuf,
    pub file: SourceFile,
}

#[derive(Clone, Debug)]
pub enum Level {
    Error,
    #[allow(dead_code)]
    Warning,
}
