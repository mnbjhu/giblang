use chumsky::{error::Rich, extra, select, Parser};

use crate::{
    lexer::{literal::Literal, token::Token},
    util::{ParserInput, Span},
    AstParser,
};

use self::code_block::{code_block_parser, CodeBlock};

use super::stmt::Stmt;

pub mod code_block;

#[derive(Clone, PartialEq, Debug)]
pub enum Expr {
    Literal(Literal),
    Ident(String),
    CodeBlock(CodeBlock),
}

pub fn expr_parser<'tokens, 'src: 'tokens>(
    stmt: impl chumsky::Parser<
            'tokens,
            ParserInput<'tokens, 'src>,
            Stmt,
            extra::Err<Rich<'tokens, Token, Span>>,
        > + Clone
        + 'tokens,
) -> AstParser!(Expr) {
    let block = code_block_parser(stmt);

    let atom = select! {
        Token::Literal(lit) => Expr::Literal(lit),
        Token::Ident(ident) => Expr::Ident(ident),
    };

    block.map(Expr::CodeBlock).or(atom)
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_parse_eq,
        lexer::literal::Literal,
        parser::{
            expr::{expr_parser, Expr},
            stmt::stmt_parser,
        },
    };

    #[test]
    fn test_ident() {
        assert_parse_eq!(
            expr_parser(stmt_parser()),
            "thing",
            Expr::Ident("thing".to_string())
        );
    }

    #[test]
    fn test_literal() {
        assert_parse_eq!(expr_parser(stmt_parser()), "42", Expr::Literal(42.into()));
        assert_parse_eq!(
            expr_parser(stmt_parser()),
            "42.0",
            Expr::Literal(Literal::Float("42.0".to_string()))
        );
        assert_parse_eq!(
            expr_parser(stmt_parser()),
            "true",
            Expr::Literal(true.into())
        );
        assert_parse_eq!(
            expr_parser(stmt_parser()),
            "false",
            Expr::Literal(false.into())
        );
    }
}
