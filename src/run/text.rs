use std::{collections::HashMap, fmt::Display};

use chumsky::{
    error::Rich,
    extra,
    primitive::{choice, end, just, none_of, one_of},
    select, text, IterParser, Parser,
};
use ByteCodeKeyword::*;
use ByteCodeToken::*;

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
    Neq,
    Not,
    And,
    Or,
    Match,
    Jmp,
    Je,
    Jne,
    Index,
    SetIndex,
    Gt,
    Lt,
    Gte,
    Lte,
    Clone,
    VecGet,
    VecSet,
    VecPush,
    VecPop,
    VecLen,
    VecInsert,
    VecRemove,
    Dyn,
    DynCall,
}

pub fn byte_code_lexer<'src>(
) -> impl Parser<'src, &'src str, Vec<Spanned<ByteCodeToken>>, extra::Err<Rich<'src, char>>> {
    let label = just('$')
        .ignore_then(text::ident())
        .map(str::to_string)
        .map(ByteCodeToken::Label);

    let ident = text::ident().map(|ident| match ident {
        "func" => Keyword(ByteCodeKeyword::Func),
        "push" => Keyword(Push),
        "pop" => Keyword(ByteCodeKeyword::Pop),
        "print" => Keyword(ByteCodeKeyword::Print),
        "panic" => Keyword(Panic),
        "construct" => Keyword(Construct),
        "call" => Keyword(Call),
        "return" => Keyword(Return),
        "new" => Keyword(NewLocal),
        "get" => Keyword(GetLocal),
        "set" => Keyword(SetLocal),
        "goto" => Keyword(Goto),
        "param" => Keyword(Param),
        "mul" => Keyword(Mul),
        "add" => Keyword(Add),
        "sub" => Keyword(Sub),
        "eq" => Keyword(Eq),
        "neq" => Keyword(Neq),
        "and" => Keyword(And),
        "or" => Keyword(Or),
        "not" => Keyword(Not),
        "match" => ByteCodeToken::Keyword(Match),
        "je" => Keyword(Je),
        "jne" => Keyword(Jne),
        "jmp" => Keyword(Jmp),
        "copy" => Keyword(Copy),
        "index" => Keyword(Index),
        "set_index" => Keyword(SetIndex),
        "gt" => Keyword(Gt),
        "lt" => Keyword(Lt),
        "gte" => Keyword(Gte),
        "lte" => Keyword(Lte),
        "clone" => Keyword(Clone),
        "vec_get" => Keyword(VecGet),
        "vec_set" => Keyword(VecSet),
        "vec_push" => Keyword(VecPush),
        "vec_pop" => Keyword(VecPop),
        "vec_len" => Keyword(VecLen),
        "vec_insert" => Keyword(VecInsert),
        "vec_remove" => Keyword(VecRemove),
        "dyn" => Keyword(Dyn),
        "dyn_call" => Keyword(DynCall),
        "true" => Literal(Literal::Bool(true)),
        "false" => Literal(Literal::Bool(false)),
        _ => Ident(ident.to_string()),
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
> + core::clone::Clone
       + 'tokens {
    just(ByteCodeToken::Keyword(kw)).ignored()
}
impl Display for ByteCodeToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Keyword(k) => write!(f, "{k}"),
            Ident(i) => write!(f, "{i}"),
            Literal(l) => write!(f, "{l}"),
            Op(o) => write!(f, "{o}"),
            Punct(p) => write!(f, "{p}"),
            Newline => write!(f, "newline"),
            Label(l) => write!(f, "${l}"),
        }
    }
}

impl Display for ByteCodeKeyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ByteCodeKeyword::Func => write!(f, "func"),
            Push => write!(f, "push"),
            Pop => write!(f, "pop"),
            Print => write!(f, "print"),
            Panic => write!(f, "panic"),
            Construct => write!(f, "construct"),
            Call => write!(f, "call"),
            Return => write!(f, "return"),
            NewLocal => write!(f, "new"),
            GetLocal => write!(f, "get"),
            SetLocal => write!(f, "set"),
            Goto => write!(f, "goto"),
            Param => write!(f, "param"),
            Mul => write!(f, "mul"),
            Add => write!(f, "add"),
            Sub => write!(f, "sub"),
            Eq => write!(f, "eq"),
            Neq => write!(f, "neq"),
            Not => write!(f, "not"),
            And => write!(f, "and"),
            Or => write!(f, "or"),
            Match => write!(f, "match"),
            Jmp => write!(f, "jmp"),
            Je => write!(f, "je"),
            Jne => write!(f, "jne"),
            Copy => write!(f, "copy"),
            Index => write!(f, "index"),
            SetIndex => write!(f, "set_index"),
            Gt => write!(f, "gt"),
            Lt => write!(f, "lt"),
            Gte => write!(f, "gte"),
            Lte => write!(f, "lte"),
            Clone => write!(f, "clone"),
            VecGet => write!(f, "vec_get"),
            VecSet => write!(f, "vec_set"),
            VecPush => write!(f, "vec_push"),
            VecPop => write!(f, "vec_pop"),
            VecLen => write!(f, "vec_len"),
            VecInsert => write!(f, "vec_insert"),
            VecRemove => write!(f, "vec_remove"),
            Dyn => write!(f, "dyn"),
            DynCall => write!(f, "dyn_call"),
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
> + core::clone::Clone
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
> + core::clone::Clone
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
    let basic = select! {
        ByteCodeToken::Keyword(ByteCodeKeyword::Copy) => ByteCode::Copy,
        ByteCodeToken::Keyword(ByteCodeKeyword::Mul) => ByteCode::Mul,
        ByteCodeToken::Keyword(ByteCodeKeyword::Add) => ByteCode::Add,
        ByteCodeToken::Keyword(ByteCodeKeyword::Sub) => ByteCode::Sub,
        ByteCodeToken::Keyword(ByteCodeKeyword::Not) => ByteCode::Not,
        ByteCodeToken::Keyword(ByteCodeKeyword::Eq) => ByteCode::Eq,
        ByteCodeToken::Keyword(ByteCodeKeyword::And) => ByteCode::And,
        ByteCodeToken::Keyword(ByteCodeKeyword::Or) => ByteCode::Or,
        ByteCodeToken::Keyword(ByteCodeKeyword::Gt) => ByteCode::Gt,
        ByteCodeToken::Keyword(ByteCodeKeyword::Lt) => ByteCode::Lt,
        ByteCodeToken::Keyword(ByteCodeKeyword::Gte) => ByteCode::Gte,
        ByteCodeToken::Keyword(ByteCodeKeyword::Lte) => ByteCode::Lte,
        ByteCodeToken::Keyword(ByteCodeKeyword::Clone) => ByteCode::Clone,
        ByteCodeToken::Keyword(ByteCodeKeyword::VecGet) => ByteCode::VecGet,
        ByteCodeToken::Keyword(ByteCodeKeyword::VecSet) => ByteCode::VecSet,
        ByteCodeToken::Keyword(ByteCodeKeyword::VecPush) => ByteCode::VecPush,
        ByteCodeToken::Keyword(ByteCodeKeyword::VecPop) => ByteCode::VecPop,
        ByteCodeToken::Keyword(ByteCodeKeyword::VecLen) => ByteCode::VecLen,
        ByteCodeToken::Keyword(ByteCodeKeyword::VecInsert) => ByteCode::VecInsert,
        ByteCodeToken::Keyword(ByteCodeKeyword::VecRemove) => ByteCode::VecRemove,
    };

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
    let dyn_call = keyword(ByteCodeKeyword::DynCall)
        .ignore_then(num)
        .map(ByteCode::SetIndex);
    let dyn_ = keyword(ByteCodeKeyword::Dyn)
        .ignore_then(num)
        .map(ByteCode::SetIndex);

    let int = just(ByteCodeToken::Op("-".to_string()))
        .or_not()
        .then(num)
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
        choice((
            pop, push, print, panic, construct, call, ret, new_local, get_local, set_local, goto,
        )),
        choice((
            param, match_, jmp, je, jne, index, set_index, dyn_, dyn_call,
        )),
        basic,
    ))
}

pub fn bc_parser<'tokens, 'src: 'tokens>() -> impl Parser<
    'tokens,
    ByteCodeParserInput<'tokens, 'src>,
    HashMap<u32, FuncDef>,
    extra::Full<Rich<'tokens, ByteCodeToken, Span>, (), ()>,
> + core::clone::Clone
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
