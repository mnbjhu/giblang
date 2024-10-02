use chumsky::primitive::just;
use chumsky::Parser;

use crate::lexer::token::punct;
use crate::parser::common::generic_args::{generic_args_parser, GenericArgs};
use crate::parser::common::ident::spanned_ident_parser;
use crate::parser::common::type_::type_parser;
use crate::parser::expr::code_block::code_block_parser;
use crate::parser::stmt::Stmt;
use crate::{kw, AstParser};
use crate::{parser::common::type_::Type, util::Spanned};

use super::arg::{function_args_parser, FunctionArg};

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Func {
    pub receiver: Option<Spanned<Type>>,
    pub name: Spanned<String>,
    pub args: Vec<Spanned<FunctionArg>>,
    pub generics: GenericArgs,
    pub ret: Option<Spanned<Type>>,
    pub body: Option<Vec<Spanned<Stmt>>>,
}

pub fn func_parser<'tokens, 'src: 'tokens>(stmt: AstParser!(Stmt)) -> AstParser!(Func) {
    let receiver = type_parser()
        .map_with(|t, e| (t, e.span()))
        .then_ignore(just(punct('.')))
        .or_not();

    let ret = just(punct(':'))
        .ignore_then(type_parser().map_with(|t, e| (t, e.span())))
        .or_not();

    just(kw!(fn))
        .ignore_then(receiver)
        .then(spanned_ident_parser())
        .then(generic_args_parser())
        .then(function_args_parser())
        .then(ret)
        .then(code_block_parser(stmt).or_not())
        .map(|(((((receiver, name), generics), args), ret), body)| Func {
            receiver,
            name,
            args,
            generics,
            ret,
            body,
        })
}
