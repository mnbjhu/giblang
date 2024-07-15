use chumsky::{
    primitive::{choice, just},
    recursive::recursive,
    select, Parser,
};

use crate::{
    lexer::{
        literal::Literal,
        token::{punct, Token},
    },
    parser::expr::match_::{match_parser, Match},
    AstParser,
};

use self::{
    call::{call_parser, Call},
    code_block::{code_block_parser, CodeBlock},
    qualified_name::{qualified_name_parser, SpannedQualifiedName},
};

use super::stmt::Stmt;

pub mod call;
pub mod code_block;
pub mod match_;
pub mod match_arm;
pub mod qualified_name;

#[derive(Clone, PartialEq, Debug)]
pub enum Expr {
    Literal(Literal),
    Ident(SpannedQualifiedName),
    CodeBlock(CodeBlock),
    Call(Call),
    Match(Match),
}

pub fn expr_parser<'tokens, 'src: 'tokens>(stmt: AstParser!(Stmt)) -> AstParser!(Expr) {
    let block = code_block_parser(stmt).map(Expr::CodeBlock);

    recursive(|expr| {
        let atom = select! {
            Token::Literal(lit) => Expr::Literal(lit),
        }
        .or(qualified_name_parser().map(Expr::Ident))
        .or(expr
            .clone()
            .delimited_by(just(punct('(')), just(punct('('))));

        let match_ =
            match_parser(expr.clone(), match_arm::match_arm_parser(expr.clone())).map(Expr::Match);

        let call = call_parser(atom.clone(), expr).map(Expr::Call);

        choice((match_, block, call, atom))
    })
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
