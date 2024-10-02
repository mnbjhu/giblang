use chumsky::{recursive::recursive, Parser};

use crate::AstParser;

use self::let_::{let_parser, LetStatement};

use super::expr::{expr_parser, Expr};

pub mod let_;

#[derive(Clone, PartialEq, Debug, Eq)]
pub enum Stmt {
    Let(LetStatement),
    Expr(Expr),
}

#[must_use]
pub fn stmt_parser<'tokens, 'src: 'tokens>() -> AstParser!(Stmt) {
    recursive(|stmt| {
        let let_ = let_parser(expr_parser(stmt.clone())).map(Stmt::Let);
        let expr = expr_parser(stmt).map(Stmt::Expr);
        let_.or(expr)
    })
}
#[cfg(test)]
mod tests {
    use crate::{
        assert_parse_eq,
        parser::{
            common::pattern::Pattern,
            expr::{expr_parser, Expr},
            stmt::{let_::LetStatement, stmt_parser, Stmt},
        },
    };

    #[test]
    fn test_expr_literal() {
        assert_parse_eq!(
            expr_parser(stmt_parser()),
            "thing",
            Expr::Ident(vec![("thing".to_string(), (0..5).into())])
        );

        assert_parse_eq!(expr_parser(stmt_parser()), "42", Expr::Literal(42.into()));
    }

    #[test]
    fn test_let_statement() {
        assert_parse_eq!(
            stmt_parser(),
            "let x = 42",
            Stmt::Let(LetStatement {
                pattern: (Pattern::Name("x".to_string()), (4..5).into()),
                ty: None,
                value: (Expr::Literal(42.into()), (8..10).into())
            })
        );
    }
}
