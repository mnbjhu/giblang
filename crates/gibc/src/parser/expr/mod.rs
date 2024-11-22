use access::{access_parser, basic_access_parser, Access};
use call::basic_call_parser;
use chumsky::{
    primitive::{choice, just},
    recursive::recursive,
    select, IterParser, Parser,
};
use field::Field;

use gvm::format::literal::Literal;
use lambda::{lambda_parser, Lambda};
use op::{op_parser, Op};
use while_::{while_parser, While};

use crate::{
    lexer::token::{punct, Token},
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
pub mod while_;

#[derive(Clone, PartialEq, Debug)]
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
    While(While),
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
            .map(Expr::Tuple)
            .boxed();

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
        let basic_call = basic_call_parser(atom.clone(), expr.clone()).map(Expr::Call);
        let basic_atom = choice((basic_call, atom.clone()));
        let atom = choice((call, atom));

        let basic_access = basic_atom
            .clone()
            .map_with(|ex, e| (ex, e.span()))
            .foldl_with(
                basic_access_parser(expr.clone()).repeated(),
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
            .map(|(ex, _)| ex)
            .boxed();

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
            .map(|(ex, _)| ex)
            .boxed();

        let op = op_parser(access);
        let basic_op = op_parser(basic_access);

        let match_ = match_parser(
            expr.clone(),
            match_arm::match_arm_parser(expr.clone(), stmt.clone()),
        )
        .map(Expr::Match);

        let while_ = while_parser(basic_op.clone(), stmt.clone());

        let if_else = if_else_parser(basic_op, stmt).map(Expr::IfElse);

        choice((if_else, match_, while_, lambda, op)).boxed()
    })
}
