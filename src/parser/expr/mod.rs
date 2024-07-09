use chumsky::{select, Parser};

use crate::{
    lexer::{literal::Literal, token::Token},
    AstParser,
};

use self::{
    code_block::{code_block_parser, CodeBlock},
    qualified_name::{qualified_name_parser, SpannedQualifiedName},
};

use super::stmt::Stmt;

pub mod code_block;
pub mod qualified_name;

#[derive(Clone, PartialEq, Debug)]
pub enum Expr {
    Literal(Literal),
    Ident(SpannedQualifiedName),
    CodeBlock(CodeBlock),
}

pub fn expr_parser<'tokens, 'src: 'tokens>(stmt: AstParser!(Stmt)) -> AstParser!(Expr) {
    let block = code_block_parser(stmt);

    let atom = select! {
        Token::Literal(lit) => Expr::Literal(lit),
    }
    .or(qualified_name_parser().map(Expr::Ident));

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
            Expr::Ident(vec![("thing".to_string(), (0..5).into())])
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
