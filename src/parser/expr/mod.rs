use access::{access_parser, Access};
use chumsky::{
    primitive::{choice, just},
    recursive::recursive,
    select, IterParser, Parser,
};
use field::Field;

use lambda::{lambda_parser, Lambda};
use op::{op_parser, Op};

use crate::{
    lexer::{
        literal::Literal,
        token::{punct, Token},
    },
    parser::expr::match_::{match_parser, Match},
    util::Spanned,
    AstParser,
};

use self::{
    call::{call_parser, Call},
    code_block::CodeBlock,
    if_else::{if_else_parser, IfElse},
    member::MemberCall,
    qualified_name::{qualified_name_parser, SpannedQualifiedName},
};

use super::{common::optional_newline::optional_newline, stmt::Stmt};

pub mod access;
pub mod call;
pub mod code_block;
pub mod field;
pub mod if_else;
pub mod lambda;
pub mod match_;
pub mod match_arm;
pub mod member;
pub mod op;
pub mod qualified_name;

#[derive(Clone, PartialEq, Debug, Eq)]
pub enum Expr {
    Literal(Literal),
    Field(Field),
    Ident(SpannedQualifiedName),
    CodeBlock(CodeBlock),
    Call(Call),
    MemberCall(MemberCall),
    Match(Match),
    Tuple(Vec<Spanned<Expr>>),
    IfElse(IfElse),
    Op(Op),
    Lambda(Lambda),
    Error,
}

pub fn expr_parser<'tokens, 'src: 'tokens>(stmt: AstParser!(Stmt)) -> AstParser!(Expr) {
    // let block = code_block_parser(stmt.clone()).map(Expr::CodeBlock);
    let lambda = lambda_parser(stmt.clone()).map(Expr::Lambda);

    recursive(|expr| {
        let tuple = expr
            .clone()
            .map_with(|ex, e| (ex, e.span()))
            .separated_by(just(punct(',')).padded_by(optional_newline()))
            .allow_trailing()
            .collect()
            .delimited_by(
                just(punct('(')).then(optional_newline()),
                optional_newline().then(just(punct(')'))),
            )
            .map(Expr::Tuple);

        let bracketed = expr
            .clone()
            .delimited_by(just(punct('(')), just(punct(')')));

        let atom = select! {
            Token::Literal(lit) => Expr::Literal(lit),
        }
        .or(qualified_name_parser().map(Expr::Ident))
        .or(bracketed)
        .or(tuple);

        let call = call_parser(atom.clone(), expr.clone(), lambda.clone()).map(Expr::Call);
        let atom = choice((call, atom));

        let access = atom
            .clone()
            .map_with(|ex, e| (ex, e.span()))
            .foldl_with(
                access_parser(expr.clone(), lambda.clone()).repeated(),
                |ex, acc, e| match acc {
                    Access::Field(name) => (
                        Expr::Field(Field {
                            name,
                            struct_: Box::new(ex),
                        }),
                        e.span(),
                    ),
                    Access::Member { name, args } => (
                        Expr::MemberCall(MemberCall {
                            rec: Box::new(ex),
                            name,
                            args,
                        }),
                        e.span(),
                    ),
                },
            )
            .map(|(ex, _)| ex);

        let op = op_parser(access);

        let match_ = match_parser(
            expr.clone(),
            match_arm::match_arm_parser(expr.clone(), stmt.clone()),
        )
        .map(Expr::Match);

        let if_else = if_else_parser(expr, stmt).map(Expr::IfElse);

        choice((if_else, match_, lambda, op))
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
