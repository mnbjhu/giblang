use std::path::PathBuf;

use crate::util::Span;

#[salsa::accumulator]
pub struct Diagnostic {
    pub message: String,
    pub span: Span,
    pub level: Level,
    pub path: PathBuf,
}

#[derive(Clone, Debug)]
pub enum Level {
    Error,
    Warning,
}
