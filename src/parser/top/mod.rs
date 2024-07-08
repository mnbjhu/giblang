use chumsky::{primitive::choice, Parser};

use crate::{parser::stmt::stmt_parser, AstParser};

pub mod arg;
pub mod func;
pub mod impl_;
pub mod struct_;
pub mod struct_field;
pub mod trait_;

#[derive(Debug, PartialEq, Clone)]
pub enum Top {
    Func(func::Func),
    Struct(struct_::Struct),
}

pub fn top_parser<'tokens, 'src: 'tokens>() -> AstParser!(Top) {
    choice((
        func::func_parser(stmt_parser()).map(Top::Func),
        struct_::struct_parser().map(Top::Struct),
    ))
}
