use crate::util::Span;

pub struct Diag {
    pub span: Span,
    pub message: String,
    pub kind: DiagKind,
}

pub enum DiagKind {
    Error,
    Warning,
    Info,
}

impl Diag {
    pub fn error(span: Span, message: impl Into<String>) -> Self {
        Self {
            span,
            message: message.into(),
            kind: DiagKind::Error,
        }
    }
}
