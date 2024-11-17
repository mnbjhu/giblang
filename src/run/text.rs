use std::{collections::HashMap, fmt::Display};

use chumsky::{
    error::Rich,
    extra,
    primitive::{choice, end, just, none_of, one_of},
    select, text, IterParser, Parser,
};

use crate::{
    lexer::literal::Literal,
    util::{Span, Spanned},
};

use super::{bytecode::ByteCode, state::FuncDef};

#[derive(Debug, PartialEq, Clone)]
pub enum ByteCodeToken {
    Keyword(ByteCodeKeyword),
    Label(String),
    Ident(String),
    Literal(Literal),
    Op(String),
    Punct(char),
    Newline,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ByteCodeKeyword {
    Copy,
    Func,
    Push,
    Pop,
    Print,
    Panic,
    Construct,
    Call,
    Return,
    NewLocal,
    GetLocal,
    SetLocal,
    Goto,
    Param,
    Mul,
    Add,
    Sub,
    Eq,
    Not,
    And,
    Or,
    Match,
    Jmp,
    Je,
    Jne,
    Index,
    SetIndex,
}

pub fn byte_code_lexer<'src>(
) -> impl Parser<'src, &'src str, Vec<Spanned<ByteCodeToken>>, extra::Err<Rich<'src, char>>> {
    let label = just('$')
        .ignore_then(text::ident())
        .map(str::to_string)
        .map(ByteCodeToken::Label);

    let ident = text::ident().map(|ident| match ident {
        "func" => ByteCodeToken::Keyword(ByteCodeKeyword::Func),
        "push" => ByteCodeToken::Keyword(ByteCodeKeyword::Push),
        "pop" => ByteCodeToken::Keyword(ByteCodeKeyword::Pop),
        "print" => ByteCodeToken::Keyword(ByteCodeKeyword::Print),
        "panic" => ByteCodeToken::Keyword(ByteCodeKeyword::Panic),
        "construct" => ByteCodeToken::Keyword(ByteCodeKeyword::Construct),
        "call" => ByteCodeToken::Keyword(ByteCodeKeyword::Call),
        "return" => ByteCodeToken::Keyword(ByteCodeKeyword::Return),
        "new" => ByteCodeToken::Keyword(ByteCodeKeyword::NewLocal),
        "get" => ByteCodeToken::Keyword(ByteCodeKeyword::GetLocal),
        "set" => ByteCodeToken::Keyword(ByteCodeKeyword::SetLocal),
        "goto" => ByteCodeToken::Keyword(ByteCodeKeyword::Goto),
        "param" => ByteCodeToken::Keyword(ByteCodeKeyword::Param),
        "mul" => ByteCodeToken::Keyword(ByteCodeKeyword::Mul),
        "add" => ByteCodeToken::Keyword(ByteCodeKeyword::Add),
        "sub" => ByteCodeToken::Keyword(ByteCodeKeyword::Sub),
        "eq" => ByteCodeToken::Keyword(ByteCodeKeyword::Eq),
        "and" => ByteCodeToken::Keyword(ByteCodeKeyword::And),
        "or" => ByteCodeToken::Keyword(ByteCodeKeyword::Or),
        "not" => ByteCodeToken::Keyword(ByteCodeKeyword::Not),
        "match" => ByteCodeToken::Keyword(ByteCodeKeyword::Match),
        "je" => ByteCodeToken::Keyword(ByteCodeKeyword::Je),
        "jne" => ByteCodeToken::Keyword(ByteCodeKeyword::Jne),
        "jmp" => ByteCodeToken::Keyword(ByteCodeKeyword::Jmp),
        "copy" => ByteCodeToken::Keyword(ByteCodeKeyword::Copy),
        "index" => ByteCodeToken::Keyword(ByteCodeKeyword::Index),
        "set_index" => ByteCodeToken::Keyword(ByteCodeKeyword::SetIndex),
        "true" => ByteCodeToken::Literal(Literal::Bool(true)),
        "false" => ByteCodeToken::Literal(Literal::Bool(false)),
        _ => ByteCodeToken::Ident(ident.to_string()),
    });

    let string = none_of("\"")
        .repeated()
        .to_slice()
        .map(|s: &str| ByteCodeToken::Literal(Literal::String(s.to_string())))
        .delimited_by(just('"'), just('"'));

    let digits = text::digits(10).repeated().at_least(1);

    let float = digits
        .then(just('.'))
        .then(digits)
        .to_slice()
        .map(|s: &str| ByteCodeToken::Literal(Literal::Float(s.to_string())));

    let int = digits
        .to_slice()
        .map(|s: &str| ByteCodeToken::Literal(Literal::Int(s.to_string())));

    let char = none_of('\'')
        .delimited_by(just('\''), just('\''))
        .map(|c: char| ByteCodeToken::Literal(Literal::Char(c)));

    let op = one_of("+-*/=<>_")
        .repeated()
        .at_least(1)
        .to_slice()
        .map(|s: &str| ByteCodeToken::Op(s.to_string()));

    let punct = one_of("(){}[],.:;").map(ByteCodeToken::Punct);

    let whitespace = one_of(" \t").repeated();

    let comment = just("//")
        .then(none_of('\n').repeated())
        .then(just('\n'))
        .ignored();

    let newline = comment
        .or(just('\n').ignored())
        .padded_by(whitespace)
        .repeated()
        .at_least(1)
        .map(|()| ByteCodeToken::Newline);

    choice((newline, label, ident, char, float, int, string, op, punct))
        .map_with(|t, e| (t, e.span()))
        .padded_by(whitespace)
        .repeated()
        .collect()
        .then_ignore(end())
}

fn keyword<'tokens, 'src>(
    kw: ByteCodeKeyword,
) -> impl Parser<
    'tokens,
    ByteCodeParserInput<'tokens, 'src>,
    (),
    extra::Full<Rich<'tokens, ByteCodeToken, Span>, (), ()>,
> + Clone
       + 'tokens {
    just(ByteCodeToken::Keyword(kw)).ignored()
}
impl Display for ByteCodeToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ByteCodeToken::Keyword(k) => write!(f, "{k}"),
            ByteCodeToken::Ident(i) => write!(f, "{i}"),
            ByteCodeToken::Literal(l) => write!(f, "{l}"),
            ByteCodeToken::Op(o) => write!(f, "{o}"),
            ByteCodeToken::Punct(p) => write!(f, "{p}"),
            ByteCodeToken::Newline => write!(f, "newline"),
            ByteCodeToken::Label(l) => write!(f, "${l}"),
        }
    }
}

impl Display for ByteCodeKeyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ByteCodeKeyword::Func => write!(f, "func"),
            ByteCodeKeyword::Push => write!(f, "push"),
            ByteCodeKeyword::Pop => write!(f, "pop"),
            ByteCodeKeyword::Print => write!(f, "print"),
            ByteCodeKeyword::Panic => write!(f, "panic"),
            ByteCodeKeyword::Construct => write!(f, "construct"),
            ByteCodeKeyword::Call => write!(f, "call"),
            ByteCodeKeyword::Return => write!(f, "return"),
            ByteCodeKeyword::NewLocal => write!(f, "new"),
            ByteCodeKeyword::GetLocal => write!(f, "get"),
            ByteCodeKeyword::SetLocal => write!(f, "set"),
            ByteCodeKeyword::Goto => write!(f, "goto"),
            ByteCodeKeyword::Param => write!(f, "param"),
            ByteCodeKeyword::Mul => write!(f, "mul"),
            ByteCodeKeyword::Add => write!(f, "add"),
            ByteCodeKeyword::Sub => write!(f, "sub"),
            ByteCodeKeyword::Eq => write!(f, "eq"),
            ByteCodeKeyword::Not => write!(f, "not"),
            ByteCodeKeyword::And => write!(f, "and"),
            ByteCodeKeyword::Or => write!(f, "or"),
            ByteCodeKeyword::Match => write!(f, "match"),
            ByteCodeKeyword::Jmp => write!(f, "jmp"),
            ByteCodeKeyword::Je => write!(f, "je"),
            ByteCodeKeyword::Jne => write!(f, "jne"),
            ByteCodeKeyword::Copy => write!(f, "copy"),
            ByteCodeKeyword::Index => write!(f, "index"),
            ByteCodeKeyword::SetIndex => write!(f, "set_index"),
        }
    }
}

pub type ByteCodeParserInput<'tokens, 'src> =
    chumsky::input::SpannedInput<ByteCodeToken, Span, &'tokens [(ByteCodeToken, Span)]>;

pub fn bc_func_parser<'tokens, 'src: 'tokens>() -> impl Parser<
    'tokens,
    ByteCodeParserInput<'tokens, 'src>,
    (u32, u32),
    extra::Full<Rich<'tokens, ByteCodeToken, Span>, (), ()>,
> + Clone
       + 'tokens {
    let num = select! {
        ByteCodeToken::Literal(Literal::Int(n)) => n.parse().unwrap(),
    };
    let args = num
        .separated_by(just(ByteCodeToken::Punct(',')))
        .exactly(2)
        .collect::<Vec<_>>()
        .map(|args| (args[0], args[1]));
    let func = keyword(ByteCodeKeyword::Func).ignore_then(args);
    func
}

pub fn bc_op_parser<'tokens, 'src: 'tokens>() -> impl Parser<
    'tokens,
    ByteCodeParserInput<'tokens, 'src>,
    ByteCode,
    extra::Full<Rich<'tokens, ByteCodeToken, Span>, (), ()>,
> + Clone
       + 'tokens {
    let num = select! {
        ByteCodeToken::Literal(Literal::Int(n)) => {
            let num: u32 = n.parse().unwrap();
            num
        },
    };
    let literal = select! {
        ByteCodeToken::Literal(l) => l,
    };
    let pop = keyword(ByteCodeKeyword::Pop).map(|()| ByteCode::Pop);
    let push = keyword(ByteCodeKeyword::Push)
        .ignore_then(literal)
        .map(ByteCode::Push);
    let print = keyword(ByteCodeKeyword::Print).map(|()| ByteCode::Print);
    let panic = keyword(ByteCodeKeyword::Panic).map(|()| ByteCode::Panic);
    let construct = keyword(ByteCodeKeyword::Construct)
        .ignore_then(
            num.separated_by(just(ByteCodeToken::Punct(',')))
                .exactly(2)
                .collect::<Vec<_>>(),
        )
        .map(|args| {
            let id = args[0];
            let len = args[1];
            ByteCode::Construct { id, len }
        });
    let call = keyword(ByteCodeKeyword::Call)
        .ignore_then(num)
        .map(ByteCode::Call);
    let copy = keyword(ByteCodeKeyword::Copy).map(|()| ByteCode::Copy);
    let mul = keyword(ByteCodeKeyword::Mul).map(|()| ByteCode::Mul);
    let add = keyword(ByteCodeKeyword::Add).map(|()| ByteCode::Add);
    let sub = keyword(ByteCodeKeyword::Sub).map(|()| ByteCode::Sub);
    let not = keyword(ByteCodeKeyword::Not).map(|()| ByteCode::Not);
    let eq = keyword(ByteCodeKeyword::Eq).map(|()| ByteCode::Eq);
    let and = keyword(ByteCodeKeyword::And).map(|()| ByteCode::And);
    let or = keyword(ByteCodeKeyword::Or).map(|()| ByteCode::Or);

    let ret = keyword(ByteCodeKeyword::Return).map(|()| ByteCode::Return);
    let param = keyword(ByteCodeKeyword::Param)
        .ignore_then(num)
        .map(ByteCode::Param);
    let match_ = keyword(ByteCodeKeyword::Match)
        .ignore_then(num)
        .map(ByteCode::Match);
    let index = keyword(ByteCodeKeyword::Index)
        .ignore_then(num)
        .map(ByteCode::Index);
    let set_index = keyword(ByteCodeKeyword::SetIndex)
        .ignore_then(num)
        .map(ByteCode::SetIndex);

    let int = just(ByteCodeToken::Op("-".to_string()))
        .or_not()
        .then(num.clone())
        .map(|(s, n)| match s {
            Some(_) => -(n as i32),
            None => n as i32,
        });
    let jmp = keyword(ByteCodeKeyword::Jmp)
        .ignore_then(int.clone())
        .map(ByteCode::Jmp);

    let je = keyword(ByteCodeKeyword::Je)
        .ignore_then(int.clone())
        .map(ByteCode::Je);

    let jne = keyword(ByteCodeKeyword::Jne)
        .ignore_then(int)
        .map(ByteCode::Jne);

    let get_local = keyword(ByteCodeKeyword::GetLocal)
        .ignore_then(num)
        .map(ByteCode::GetLocal);
    let new_local = keyword(ByteCodeKeyword::NewLocal)
        .ignore_then(num)
        .map(ByteCode::NewLocal);

    let set_local = keyword(ByteCodeKeyword::SetLocal)
        .ignore_then(num)
        .map(ByteCode::SetLocal);

    let goto = keyword(ByteCodeKeyword::Goto)
        .ignore_then(num)
        .map(ByteCode::Goto);

    choice((
        pop, push, print, panic, construct, call, ret, new_local, get_local, set_local, goto,
        param, mul, add, sub, or, and, eq, not, match_, jmp, je, jne, copy, index, set_index,
    ))
}

pub fn bc_parser<'tokens, 'src: 'tokens>() -> impl Parser<
    'tokens,
    ByteCodeParserInput<'tokens, 'src>,
    HashMap<u32, FuncDef>,
    extra::Full<Rich<'tokens, ByteCodeToken, Span>, (), ()>,
> + Clone
       + 'tokens {
    bc_func_parser()
        .then_ignore(just(ByteCodeToken::Newline))
        .then(
            bc_op_parser()
                .separated_by(just(ByteCodeToken::Newline))
                .collect(),
        )
        .map_with(|((id, args), body), e| {
            (
                id,
                FuncDef {
                    args,
                    body,
                    offset: e.span().start,
                },
            )
        })
        .separated_by(just(ByteCodeToken::Newline))
        .allow_trailing()
        .collect()
}
