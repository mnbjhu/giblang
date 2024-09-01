use crate::lexer::literal::Literal;

use super::ExprKind;

impl Literal {
    pub fn build(&self, kind: &ExprKind) -> String {
        let expr = match self {
            Literal::Int(i) => i.to_string(),
            Literal::Float(f) => f.to_string(),
            Literal::String(s) => format!("\"{s}\""),
            Literal::Bool(b) => b.to_string(),
            Literal::Char(c) => format!("'{c}'"),
        };
        kind.basic_apply(expr)
    }
}
